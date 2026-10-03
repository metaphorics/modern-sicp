// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 5.5

import { type LinkedModules, type RunResult, Session } from "@sicp-ts/ch4/01-metacircular";
import { format } from "@sicp-ts/ch4/read";
import { builtinMember } from "@sicp-ts/ch4/runtime/builtins";
import { child, type Env, findCell, makeCell } from "@sicp-ts/ch4/runtime/env";
/**
 * The compiler (host-subsets grammar section 5.5): the checked host-subset
 * syntax compiles at compile time to typed register-machine instructions;
 * the simulator executes the emitted instruction data and nothing else.
 * Lambda bodies compile inline behind unique entry labels, `return` is the
 * `continue` register, and `throw`/`break`/`continue` are static jumps to
 * compile-time handler and loop labels (a pending throw crosses call frames
 * through the `thrown` register, checked at every call site).
 *
 * Needs/modifies sets follow the book so `preserving` protects registers
 * exactly across sub-compilations. Compiled operation names are the pinned
 * exercise surface shared with the chapter exercises.
 */
import { fail, type GuestError, ok } from "@sicp-ts/ch4/runtime/errors";
import {
  ArrayValue,
  Closure,
  ErrorValue,
  MapValue,
  PrimitiveProcedure,
  RecordValue,
  SetValue,
  type Value,
} from "@sicp-ts/ch4/runtime/value";
import type { Arg, Block, Decl, Expr, Stmt } from "@sicp-ts/ch4/syntax/ast";
import { admitSource } from "@sicp-ts/ch4/syntax/check";
import { type Machine, makeMachine } from "./02-simulator.ts";
import {
  assign,
  branch,
  constant,
  type MachineStatement as GenericMachineStatement,
  gotoLabel,
  gotoRegister,
  MachineErrorValue,
  type Operation,
  op,
  perform,
  register,
  restore,
  save,
  type Transfer,
  test,
  type Word,
} from "./04-eceval.ts";

export { readProgram, readProgram as parse } from "@sicp-ts/ch4/read";

type MachineStatement = GenericMachineStatement<Word>;

// ---------------------------------------------------------------------
// Instruction sequences and linkage (the book's 5.5.1–5.5.2 data).
// ---------------------------------------------------------------------

/** Where control continues after a compiled piece. */
export type Linkage =
  | { readonly kind: "next" }
  | { readonly kind: "return" }
  | { readonly kind: "goto"; readonly target: string };

/** The book's `next` linkage. */
export const nextLinkage: Linkage = { kind: "next" };
/** The book's `return` linkage: `goto` the `continue` register. */
export const returnLinkage: Linkage = { kind: "return" };
/** A direct jump to a compile-time label. */
export const gotoLinkage = (target: string): Linkage => ({ kind: "goto", target });

/** One compiled piece: register use plus emitted machine statements. */
export interface InstructionSequence {
  readonly needs: ReadonlySet<string>;
  readonly modifies: ReadonlySet<string>;
  readonly statements: ReadonlyArray<MachineStatement>;
}

/** A compilation result: instructions or a typed compile error. */
export type CompileResult = InstructionSequence | GuestError;

/** A compiled program: per-form sequences and the flat instruction stream. */
export interface CompiledProgram {
  readonly sequences: ReadonlyArray<InstructionSequence>;
  readonly instructions: ReadonlyArray<MachineStatement>;
}

/** Discriminates the compile-error arm of {@link CompileResult}. */
export const isCompileError = (result: CompileResult | CompiledProgram): result is GuestError =>
  !("statements" in result) && !("instructions" in result);

const emptySequence: InstructionSequence = {
  needs: new Set(),
  modifies: new Set(),
  statements: [],
};

const makeSequence = (
  needs: Iterable<string>,
  modifies: Iterable<string>,
  statements: ReadonlyArray<MachineStatement>,
): InstructionSequence => ({ needs: new Set(needs), modifies: new Set(modifies), statements });

const union = (...sets: ReadonlyArray<ReadonlySet<string>>): Set<string> => {
  const out = new Set<string>();
  for (const set of sets) {
    for (const item of set) {
      out.add(item);
    }
  }
  return out;
};

const minus = (set: ReadonlySet<string>, drop: ReadonlySet<string>): Set<string> => {
  const out = new Set<string>();
  for (const item of set) {
    if (!drop.has(item)) {
      out.add(item);
    }
  }
  return out;
};

/** The book's `append-instruction-sequences`. */
export const appendInstructionSequences = (
  ...sequences: ReadonlyArray<InstructionSequence>
): InstructionSequence => {
  let needs = new Set<string>();
  let modifies = new Set<string>();
  const statements: MachineStatement[] = [];
  for (const sequence of sequences) {
    needs = union(needs, minus(sequence.needs, modifies));
    modifies = union(modifies, sequence.modifies);
    statements.push(...sequence.statements);
  }
  return { needs, modifies, statements };
};

/** The book's `preserving`: saves registers a later piece still needs. */
export const preserving = (
  registers: ReadonlyArray<string>,
  first: InstructionSequence,
  second: InstructionSequence,
): InstructionSequence => {
  const protectedRegisters = registers.filter(
    (name) => second.needs.has(name) && first.modifies.has(name),
  );
  if (protectedRegisters.length === 0) {
    return appendInstructionSequences(first, second);
  }
  const saves = protectedRegisters.map((name): MachineStatement => save(name));
  const restores = [...protectedRegisters].reverse().map((name): MachineStatement => restore(name));
  return {
    needs: union(first.needs, minus(second.needs, first.modifies)),
    modifies: union(first.modifies, second.modifies),
    statements: [...saves, ...first.statements, ...restores, ...second.statements],
  };
};

let labelCounter = 0;

/** A fresh compile-time label (the book's `make-label`). */
export const makeLabel = (prefix: string): string => {
  labelCounter += 1;
  return `${prefix}-${labelCounter}`;
};

/** Resets the label counter (deterministic compilations in tests). */
export const resetLabels = (): void => {
  labelCounter = 0;
};

// ---------------------------------------------------------------------
// Compiled operations: the pinned exercise surface plus engine internals.
// ---------------------------------------------------------------------

const errorWord = (error: GuestError): Word => new MachineErrorValue(error);
const procedureEntries = new WeakMap<Closure, string>();
const isValue = (word: Word): word is Value =>
  word instanceof Closure ||
  word instanceof PrimitiveProcedure ||
  word instanceof ErrorValue ||
  word instanceof ArrayValue ||
  word instanceof RecordValue ||
  word instanceof MapValue ||
  word instanceof SetValue ||
  word === null ||
  typeof word !== "object";

const paramsOf = (word: Word): readonly string[] => (word instanceof Closure ? word.params : []);

/** The runtime table for every operation the compiler emits. */
export const compiledOperations = (
  output: string[] = [],
): Readonly<Record<string, Operation<Word>>> => {
  const truthy = (value: Word): boolean =>
    !(
      value === false ||
      value === 0 ||
      value === "" ||
      value === null ||
      value === undefined ||
      (typeof value === "number" && Number.isNaN(value))
    );
  return {
    "lookup-variable-value": (args) => {
      const name = typeof args[0] === "string" ? args[0] : "";
      const cell = findCell(args[1] as Env | null, name);
      if (cell === undefined) {
        return errorWord({ tag: "unbound-name", name });
      }
      if (!cell.initialized) {
        return errorWord({ tag: "tdz-access", name });
      }
      return cell.value;
    },
    "set-variable-value": (args) => {
      const name = typeof args[0] === "string" ? args[0] : "";
      const cell = findCell(args[2] as Env | null, name);
      if (cell === undefined) {
        return errorWord({ tag: "unbound-name", name });
      }
      if (!cell.initialized) {
        return errorWord({ tag: "tdz-access", name });
      }
      if (!cell.mutable) {
        return errorWord({
          tag: "bad-operand",
          operator: "=",
          detail: "assignment to a const binding",
        });
      }
      cell.value = args[1] as Value;
      return args[1];
    },
    "define-variable": (args) => {
      const name = typeof args[0] === "string" ? args[0] : "";
      const env = args[2] as Env | null;
      if (env === null || env === undefined) {
        return errorWord({ tag: "unbound-name", name });
      }
      const existing = env.bindings.get(name);
      if (existing !== undefined) {
        existing.value = args[1] as Value;
        existing.initialized = true;
      } else {
        env.bindings.set(name, makeCell(args[1] as Value, true));
      }
      return args[1];
    },
    "predeclare-variable": (args) => {
      const name = typeof args[0] === "string" ? args[0] : "";
      const env = args[2] as Env | null;
      if (env === null || env === undefined) {
        return errorWord({ tag: "unbound-name", name });
      }
      env.bindings.set(name, makeCell(undefined, false, args[1] === true));
      return undefined;
    },
    "extend-environment": (args) => {
      const rawParams = args[0];
      const rawValues = args[1];
      const params =
        typeof rawParams === "string" ? [rawParams] : Array.isArray(rawParams) ? rawParams : [];
      const values = Array.isArray(rawValues) ? rawValues : [rawValues];
      const frame = child((args[2] as Env | null) ?? null);
      for (const [index, param] of params.entries()) {
        frame.bindings.set(String(param), makeCell(values[index] as Value, true));
      }
      if (typeof args[3] === "string" && args[3] !== "") {
        frame.bindings.set(
          args[3],
          makeCell(new ArrayValue(values.slice(params.length) as Value[]), true),
        );
      }
      return frame;
    },
    "make-procedure": (args) => {
      const entry = typeof args[0] === "string" ? args[0] : "";
      const rawParams = args[1];
      const params = Array.isArray(rawParams) ? rawParams.map((param) => String(param)) : [];
      const rest = typeof args[3] === "string" && args[3] !== "" ? args[3] : null;
      const closure = new Closure(
        params,
        rest,
        { body: [], span: { start: 0, end: 0, line: 1, column: 1 } },
        args[2] as Env,
      );
      procedureEntries.set(closure, entry);
      return closure;
    },
    "procedure-parameters": (args) => paramsOf(args[0]) as Word,
    "procedure-rest": (args) => (args[0] instanceof Closure ? (args[0].rest ?? "") : ""),
    "procedure-environment": (args) => (args[0] instanceof Closure ? args[0].env : undefined),
    "procedure-entry": (args) => {
      const entry = args[0] instanceof Closure ? (procedureEntries.get(args[0]) ?? "") : "";
      return { tag: "symbol", name: entry } as Word;
    },
    "parent-environment": (args) => (args[0] as Env | null)?.parent ?? null,
    "record-from": (args) => {
      const keys = Array.isArray(args[0]) ? (args[0] as Value[]) : [];
      const values = Array.isArray(args[1]) ? (args[1] as Value[]) : [];
      const fields = new Map<string, Value>();
      for (const [index, key] of keys.entries()) {
        fields.set(String(key), values[index] as Value);
      }
      return new RecordValue(fields, new Set());
    },
    "typeof-value": (args) => {
      const value = args[0];
      if (typeof value === "number") {
        return "number";
      }
      if (typeof value === "string") {
        return "string";
      }
      if (typeof value === "boolean") {
        return "boolean";
      }
      if (value instanceof Closure || value instanceof PrimitiveProcedure) {
        return "function";
      }
      return value === undefined ? "undefined" : "object";
    },
    "is-primitive-procedure": (args) => args[0] instanceof PrimitiveProcedure,
    "is-compound-procedure": (args) => args[0] instanceof Closure,
    "apply-primitive-procedure": (args) => {
      const proc = args[0];
      const values = Array.isArray(args[1]) ? (args[1] as Value[]) : [];
      if (!(proc instanceof PrimitiveProcedure)) {
        return errorWord({ tag: "not-callable", detail: "unknown" });
      }
      const outcome = proc.fn(values);
      return outcome.tag === "error" ? errorWord(outcome.error) : outcome.value;
    },
    "empty-argl": () => [],
    "adjoin-arg": (args) => [
      ...(Array.isArray(args[1]) ? (args[1] as Value[]) : []),
      args[0] as Value,
    ],
    "adjoin-spread": (args) => {
      const collected = Array.isArray(args[1]) ? [...(args[1] as Value[])] : [];
      const value = args[0];
      if (!(value instanceof ArrayValue)) {
        return errorWord({
          tag: "bad-operand",
          operator: "spread",
          detail: "spread argument is not an array",
        });
      }
      return [...collected, ...value.items];
    },
    "is-error-value": (args) => args[0] instanceof MachineErrorValue,
    "error-value-message": (args) => (args[0] instanceof ErrorValue ? args[0].message : undefined),
    "throw-box": (args) =>
      args[0] instanceof MachineErrorValue
        ? ({ kind: "error", error: args[0].error } as Transfer)
        : ({ kind: "throw", value: args[0] as Value } as Transfer),
    "pending-mark": (args) => ({ kind: args[0], value: args[1] }) as unknown as Word,
    "pending-is": (args) =>
      typeof args[0] === "object" &&
      args[0] !== null &&
      "kind" in args[0] &&
      args[0].kind === args[1],
    "pending-value": (args) =>
      typeof args[0] === "object" && args[0] !== null && "value" in args[0]
        ? (args[0].value as Word)
        : undefined,
    "throw-pending": (args) => typeof args[0] === "object" && args[0] !== null && "kind" in args[0],
    "throw-error-pending": (args) =>
      typeof args[0] === "object" &&
      args[0] !== null &&
      "kind" in args[0] &&
      args[0].kind === "error",
    "throw-clear": () => undefined,
    "throw-value": (args) =>
      typeof args[0] === "object" && args[0] !== null && "value" in args[0]
        ? (args[0].value as Word)
        : undefined,
    "is-true": (args) => truthy(args[0]),
    "boolean-not": (args) => !truthy(args[0]),
    "number-add": (args) => {
      const left = args[0];
      const right = args[1];
      if (typeof left === "string" || typeof right === "string") {
        return String(left ?? "null") + String(right ?? "null");
      }
      return typeof left === "number" && typeof right === "number"
        ? left + right
        : errorWord({ tag: "bad-operand", operator: "+", detail: "non-number" });
    },
    "string-concat": (args) => String(args[0] ?? "null") + String(args[1] ?? "null"),
    "number-subtract": (args) =>
      typeof args[0] === "number" && typeof args[1] === "number"
        ? args[0] - args[1]
        : errorWord({ tag: "bad-operand", operator: "-", detail: "non-number" }),
    "number-multiply": (args) =>
      typeof args[0] === "number" && typeof args[1] === "number"
        ? args[0] * args[1]
        : errorWord({ tag: "bad-operand", operator: "*", detail: "non-number" }),
    "number-divide": (args) =>
      typeof args[0] === "number" && typeof args[1] === "number"
        ? args[0] / args[1]
        : errorWord({ tag: "bad-operand", operator: "/", detail: "non-number" }),
    "number-remainder": (args) =>
      typeof args[0] === "number" && typeof args[1] === "number"
        ? args[0] % args[1]
        : errorWord({ tag: "bad-operand", operator: "%", detail: "non-number" }),
    "number-less": (args) =>
      typeof args[0] === "number" && typeof args[1] === "number"
        ? args[0] < args[1]
        : errorWord({ tag: "bad-operand", operator: "<", detail: "non-number" }),
    "number-less-equal": (args) =>
      typeof args[0] === "number" && typeof args[1] === "number"
        ? args[0] <= args[1]
        : errorWord({ tag: "bad-operand", operator: "<=", detail: "non-number" }),
    "number-greater": (args) =>
      typeof args[0] === "number" && typeof args[1] === "number"
        ? args[0] > args[1]
        : errorWord({ tag: "bad-operand", operator: ">", detail: "non-number" }),
    "number-greater-equal": (args) =>
      typeof args[0] === "number" && typeof args[1] === "number"
        ? args[0] >= args[1]
        : errorWord({ tag: "bad-operand", operator: ">=", detail: "non-number" }),
    "value-equal": (args) => args[0] === args[1],
    "value-not-equal": (args) => args[0] !== args[1],
    "array-new": (args) =>
      new ArrayValue([...(Array.isArray(args[0]) ? (args[0] as Value[]) : [])]),
    "array-index": (args) => {
      const object = args[0];
      const index = args[1];
      if (object instanceof ArrayValue && typeof index === "number") {
        return index >= 0 && index < object.items.length ? object.items[index] : undefined;
      }
      if (typeof object === "string" && typeof index === "number") {
        const characters = [...object];
        return index >= 0 && index < characters.length ? characters[index] : undefined;
      }
      return errorWord({ tag: "bad-operand", operator: "index", detail: String(index) });
    },
    "array-set": (args) => {
      const object = args[0];
      const index = args[1];
      if (
        object instanceof ArrayValue &&
        typeof index === "number" &&
        index >= 0 &&
        index < object.items.length
      ) {
        object.items[index] = args[2] as Value;
        return args[2];
      }
      return errorWord({ tag: "bad-operand", operator: "index", detail: String(index) });
    },
    "array-length": (args) =>
      args[0] instanceof ArrayValue
        ? args[0].items.length
        : args[0] instanceof SetValue
          ? args[0].items.size
          : args[0] instanceof MapValue
            ? args[0].entries.size
            : typeof args[0] === "string"
              ? args[0].length
              : 0,
    "record-get": (args) => {
      const object = args[0];
      const name = typeof args[1] === "string" ? args[1] : "";
      if (object instanceof RecordValue && object.fields.has(name)) {
        return object.fields.get(name);
      }
      if (object instanceof ErrorValue && name === "message") {
        return object.message;
      }
      if (object instanceof ArrayValue && name === "length") {
        return object.items.length;
      }
      if (object instanceof MapValue && name === "size") {
        return object.entries.size;
      }
      if (object instanceof SetValue && name === "size") {
        return object.items.size;
      }
      if (typeof object === "string" && name === "length") {
        return object.length;
      }
      if (
        object instanceof ArrayValue ||
        object instanceof MapValue ||
        object instanceof SetValue ||
        typeof object === "string"
      ) {
        return builtinMember(object, name) ?? errorWord({ tag: "unknown-field", field: name });
      }
      return errorWord({ tag: "unknown-field", field: name });
    },
    "record-set": (args) => {
      const object = args[0];
      const name = typeof args[1] === "string" ? args[1] : "";
      if (!(object instanceof RecordValue)) {
        return errorWord({ tag: "unknown-field", field: name });
      }
      if (object.readonlyFields.has(name) && object.fields.has(name)) {
        return errorWord({ tag: "readonly-field", field: name });
      }
      object.fields.set(name, args[2] as Value);
      return args[2];
    },
    "map-new": (args) => {
      const operands = Array.isArray(args[0]) ? args[0] : [];
      const source = operands[0];
      if (source === undefined) {
        return new MapValue(new Map());
      }
      if (!(source instanceof ArrayValue)) {
        return errorWord({
          tag: "bad-operand",
          operator: "new Map",
          detail: "entries are not an array",
        });
      }
      const entries = new Map<Value, Value>();
      for (const pair of source.items) {
        if (!(pair instanceof ArrayValue) || pair.items.length !== 2) {
          return errorWord({
            tag: "bad-operand",
            operator: "new Map",
            detail: "entry is not a pair",
          });
        }
        entries.set(pair.items[0], pair.items[1]);
      }
      return new MapValue(entries);
    },
    "map-get": (args) =>
      args[0] instanceof MapValue ? args[0].entries.get(args[1] as Value) : undefined,
    "map-set": (args) => {
      if (args[0] instanceof MapValue) {
        args[0].entries.set(args[1] as Value, args[2] as Value);
      }
      return args[2];
    },
    "set-new": (args) => {
      const operands = Array.isArray(args[0]) ? args[0] : [];
      const source = operands[0];
      if (source === undefined) {
        return new SetValue(new Set());
      }
      return source instanceof ArrayValue
        ? new SetValue(new Set(source.items))
        : errorWord({ tag: "bad-operand", operator: "new Set", detail: "items are not an array" });
    },
    "error-new": (args) => {
      const operands = Array.isArray(args[0]) ? args[0] : [];
      return new ErrorValue(operands[0] === undefined ? "" : String(operands[0]));
    },
    "print-value": (args) => {
      const value = args[0] as Value;
      output.push(typeof value === "string" ? value : format(value as never));
      return undefined;
    },
  };
};

// ---------------------------------------------------------------------
// The compiler itself.
// ---------------------------------------------------------------------

interface CompileContext {
  readonly output: string[];
  readonly labels: {
    throwTarget: string | null;
    breakTarget: string | null;
    continueTarget: string | null;
    returnTarget: string | null;
  };
}

const compileError = (construct: string): GuestError => ({ tag: "unknown-syntax", construct });

const emitLinkage = (linkage: Linkage): InstructionSequence => {
  if (linkage.kind === "next") {
    return emptySequence;
  }
  if (linkage.kind === "return") {
    return makeSequence(["continue"], [], [gotoRegister("continue")]);
  }
  return makeSequence([], [], [gotoLabel(linkage.target)]);
};

const withLinkage = (body: InstructionSequence, linkage: Linkage): InstructionSequence =>
  appendInstructionSequences(body, emitLinkage(linkage));

const isConsoleLog = (expr: Expr): boolean =>
  expr.tag === "call" &&
  expr.callee.tag === "member" &&
  expr.callee.object.tag === "variable" &&
  expr.callee.object.name === "console" &&
  expr.callee.name === "log";

const binaryOperationName = (operator: string): string => {
  const names: Record<string, string> = {
    "+": "number-add",
    "-": "number-subtract",
    "*": "number-multiply",
    "/": "number-divide",
    "%": "number-remainder",
    "<": "number-less",
    "<=": "number-less-equal",
    ">": "number-greater",
    ">=": "number-greater-equal",
    "===": "value-equal",
    "!==": "value-not-equal",
  };
  return names[operator] ?? "number-add";
};

const compileOperandList = (args: ReadonlyArray<Arg>, context: CompileContext): CompileResult => {
  let collected = makeSequence(["val"], ["argl"], [assign("argl", op("empty-argl"))]);
  const raiseLabel = makeLabel("operand-raise");
  const doneLabel = makeLabel("operand-done");
  let usedSpread = false;
  for (const arg of args) {
    const piece = compileExpression(arg.expr, "val", nextLinkage, context);
    if (isCompileError(piece)) {
      return piece;
    }
    const join =
      arg.kind === "spread"
        ? makeSequence(
            ["val", "argl"],
            ["argl"],
            [
              restore("argl"),
              assign("argl", op("adjoin-spread", register("val"), register("argl"))),
              test("is-error-value", register("argl")),
              branch(raiseLabel),
            ],
          )
        : makeSequence(
            ["val", "argl"],
            ["argl"],
            [restore("argl"), assign("argl", op("adjoin-arg", register("val"), register("argl")))],
          );
    usedSpread ||= arg.kind === "spread";
    collected = appendInstructionSequences(
      collected,
      makeSequence([], ["argl"], [save("argl")]),
      piece,
      join,
    );
  }
  if (usedSpread) {
    collected = appendInstructionSequences(
      collected,
      makeSequence(
        ["argl"],
        ["val", "thrown"],
        [
          gotoLabel(doneLabel),
          { tag: "label", name: raiseLabel },
          assign("val", register("argl")),
          assign("thrown", op("throw-box", register("val"))),
          context.labels.throwTarget === null
            ? gotoRegister("continue")
            : gotoLabel(context.labels.throwTarget),
          { tag: "label", name: doneLabel },
        ],
      ),
    );
  }
  return collected;
};

/** Compiles one expression to `target` (always `val` here) under `linkage`. */
export const compileExpression = (
  expr: Expr,
  target: "val",
  linkage: Linkage,
  context: CompileContext,
): CompileResult => {
  void target;
  switch (expr.tag) {
    case "number":
    case "string":
    case "boolean": {
      const statements: MachineStatement[] = [assign("val", constant(expr.value))];
      return withLinkage(makeSequence([], ["val"], statements), linkage);
    }
    case "null":
    case "undefined": {
      const statements: MachineStatement[] = [
        assign("val", constant(expr.tag === "null" ? null : undefined)),
      ];
      return withLinkage(makeSequence([], ["val"], statements), linkage);
    }
    case "variable": {
      const statements: MachineStatement[] = [
        assign("val", op("lookup-variable-value", constant(expr.name), register("env"))),
      ];
      return withLinkage(makeSequence(["env"], ["val"], statements), linkage);
    }
    case "assign":
      return compileAssignment(expr, linkage, context);
    case "conditional":
      return compileConditional(expr, linkage, context);
    case "arrow":
      return compileLambda(expr.params, expr.body, linkage, context);
    case "call":
      return compileApplication(expr, linkage, context);
    case "binary": {
      const left = compileExpression(expr.left, "val", nextLinkage, context);
      if (isCompileError(left)) {
        return left;
      }
      const right = compileExpression(expr.right, "val", nextLinkage, context);
      if (isCompileError(right)) {
        return right;
      }
      const combine = makeSequence(
        ["val", "item"],
        ["val"],
        [
          assign("val", op(binaryOperationName(expr.op), register("item"), register("val"))),
          restore("item"),
        ],
      );
      const toItem = makeSequence(
        ["val"],
        ["item"],
        [save("item"), assign("item", register("val"))],
      );
      return withLinkage(appendInstructionSequences(left, toItem, right, combine), linkage);
    }
    case "logical": {
      const left = compileExpression(expr.left, "val", nextLinkage, context);
      if (isCompileError(left)) {
        return left;
      }
      const right = compileExpression(expr.right, "val", nextLinkage, context);
      if (isCompileError(right)) {
        return right;
      }
      const decide = makeLabel("logical-decide");
      const done = makeLabel("logical-done");
      const branchStatements: MachineStatement[] =
        expr.op === "&&"
          ? [
              test("is-true", register("val")),
              branch(decide),
              assign("val", constant(false)),
              gotoLabel(done),
            ]
          : [test("is-true", register("val")), branch(done)];
      return withLinkage(
        appendInstructionSequences(
          left,
          makeSequence(["val"], [], branchStatements),
          makeSequence([], [], [{ tag: "label", name: decide }]),
          right,
          makeSequence([], [], [{ tag: "label", name: done }]),
        ),
        linkage,
      );
    }
    case "unary": {
      const operand = compileExpression(expr.operand, "val", nextLinkage, context);
      if (isCompileError(operand)) {
        return operand;
      }
      const apply: MachineStatement[] =
        expr.op === "!"
          ? [assign("val", op("boolean-not", register("val")))]
          : expr.op === "-"
            ? [assign("val", op("number-subtract", constant(0), register("val")))]
            : expr.op === "+"
              ? []
              : [assign("val", op("typeof-value", register("val")))];
      return withLinkage(
        appendInstructionSequences(operand, makeSequence(["val"], ["val"], apply)),
        linkage,
      );
    }
    case "member": {
      const object = compileExpression(expr.object, "val", nextLinkage, context);
      if (isCompileError(object)) {
        return object;
      }
      const read = makeSequence(
        ["val"],
        ["val"],
        [assign("val", op("record-get", register("val"), constant(expr.name)))],
      );
      return withLinkage(appendInstructionSequences(object, read), linkage);
    }
    case "index": {
      const object = compileExpression(expr.object, "val", nextLinkage, context);
      if (isCompileError(object)) {
        return object;
      }
      const toObject = makeSequence(
        ["val"],
        ["item"],
        [save("item"), assign("item", register("val"))],
      );
      const index = compileExpression(expr.index, "val", nextLinkage, context);
      if (isCompileError(index)) {
        return index;
      }
      const read = makeSequence(
        ["val", "item"],
        ["val"],
        [assign("val", op("array-index", register("item"), register("val"))), restore("item")],
      );
      return withLinkage(appendInstructionSequences(object, toObject, index, read), linkage);
    }
    case "array": {
      const items = compileOperandList(expr.elements, context);
      if (isCompileError(items)) {
        return items;
      }
      return withLinkage(
        appendInstructionSequences(
          items,
          makeSequence(["argl"], ["val"], [assign("val", op("array-new", register("argl")))]),
        ),
        linkage,
      );
    }
    case "object": {
      const values = compileOperandList(
        expr.fields.map((field): Arg => ({ kind: "item", expr: field.value })),
        context,
      );
      if (isCompileError(values)) {
        return values;
      }
      const keys = expr.fields.map((field) => field.key);
      const build = makeSequence(
        ["argl"],
        ["val"],
        [
          save("item"),
          assign("item", constant(keys as Word)),
          assign("val", op("record-from", register("item"), register("argl"))),
          restore("item"),
        ],
      );
      return withLinkage(appendInstructionSequences(values, build), linkage);
    }
    case "template": {
      let collected: InstructionSequence = makeSequence(
        [],
        ["val"],
        [assign("val", constant(expr.chunks[0] ?? ""))],
      );
      for (const [index, inner] of expr.exprs.entries()) {
        const piece = compileExpression(inner, "val", nextLinkage, context);
        if (isCompileError(piece)) {
          return piece;
        }
        const join = makeSequence(
          ["val", "item"],
          ["val"],
          [
            assign("item", register("val")),
            restore("val"),
            assign("val", op("string-concat", register("val"), register("item"))),
            assign(
              "val",
              op("string-concat", register("val"), constant(expr.chunks[index + 1] ?? "")),
            ),
            restore("item"),
          ],
        );
        collected = appendInstructionSequences(
          collected,
          makeSequence([], ["val"], [save("item"), save("val")]),
          piece,
          join,
        );
      }
      return withLinkage(collected, linkage);
    }
    case "new-error": {
      const items = compileOperandList(
        expr.args.map((argument): Arg => ({ kind: "item", expr: argument })),
        context,
      );
      if (isCompileError(items)) {
        return items;
      }
      return withLinkage(
        appendInstructionSequences(
          items,
          makeSequence(["argl"], ["val"], [assign("val", op("error-new", register("argl")))]),
        ),
        linkage,
      );
    }
    case "new-map":
    case "new-set": {
      const items = compileOperandList(
        expr.args.map((argument): Arg => ({ kind: "item", expr: argument })),
        context,
      );
      if (isCompileError(items)) {
        return items;
      }
      const name = expr.tag === "new-map" ? "map-new" : "set-new";
      return withLinkage(
        appendInstructionSequences(
          items,
          makeSequence(["argl"], ["val"], [assign("val", op(name, register("argl")))]),
        ),
        linkage,
      );
    }
    default:
      return compileError(`compiled/${expr.tag as string}`);
  }
};

const compileAssignment = (
  expr: Extract<Expr, { tag: "assign" }>,
  linkage: Linkage,
  context: CompileContext,
): CompileResult => {
  const target = expr.target;
  if (target.tag === "variable") {
    const value = compileExpression(expr.value, "val", nextLinkage, context);
    if (isCompileError(value)) {
      return value;
    }
    const write = makeSequence(
      ["val", "env"],
      ["val"],
      [
        assign(
          "val",
          op("set-variable-value", constant(target.name), register("val"), register("env")),
        ),
      ],
    );
    return withLinkage(appendInstructionSequences(value, write), linkage);
  }
  if (target.tag === "member") {
    const object = compileExpression(target.object, "val", nextLinkage, context);
    if (isCompileError(object)) {
      return object;
    }
    const toObject = makeSequence(
      ["val"],
      ["proc"],
      [save("proc"), assign("proc", register("val"))],
    );
    const value = compileExpression(expr.value, "val", nextLinkage, context);
    if (isCompileError(value)) {
      return value;
    }
    const write = makeSequence(
      ["val", "proc"],
      ["val"],
      [
        assign("val", op("record-set", register("proc"), constant(target.name), register("val"))),
        restore("proc"),
      ],
    );
    return withLinkage(appendInstructionSequences(object, toObject, value, write), linkage);
  }
  if (target.tag === "index") {
    const object = compileExpression(target.object, "val", nextLinkage, context);
    if (isCompileError(object)) {
      return object;
    }
    const toObject = makeSequence(
      ["val"],
      ["item"],
      [save("item"), assign("item", register("val"))],
    );
    const index = compileExpression(target.index, "val", nextLinkage, context);
    if (isCompileError(index)) {
      return index;
    }
    const toIndex = makeSequence(
      ["val"],
      ["proc"],
      [save("proc"), assign("proc", register("val"))],
    );
    const value = compileExpression(expr.value, "val", nextLinkage, context);
    if (isCompileError(value)) {
      return value;
    }
    const write = makeSequence(
      ["val", "item", "proc"],
      ["val"],
      [
        assign("val", op("array-set", register("item"), register("proc"), register("val"))),
        restore("proc"),
        restore("item"),
      ],
    );
    return withLinkage(
      appendInstructionSequences(object, toObject, index, toIndex, value, write),
      linkage,
    );
  }
  return compileError("compiled/assign-target");
};

const compileConditional = (
  expr: Extract<Expr, { tag: "conditional" }>,
  linkage: Linkage,
  context: CompileContext,
): CompileResult => {
  const testCode = compileExpression(expr.test, "val", nextLinkage, context);
  if (isCompileError(testCode)) {
    return testCode;
  }
  const consequent = compileExpression(expr.consequent, "val", linkage, context);
  if (isCompileError(consequent)) {
    return consequent;
  }
  const alternativeBranch =
    expr.alternative === null
      ? undefined
      : compileExpression(expr.alternative, "val", linkage, context);
  if (alternativeBranch !== undefined && isCompileError(alternativeBranch)) {
    return alternativeBranch;
  }
  const trueLabel = makeLabel("true");
  const afterLabel = makeLabel("after-if");
  const alternative: InstructionSequence =
    alternativeBranch ??
    appendInstructionSequences(
      makeSequence([], ["val"], [assign("val", constant(undefined))]),
      emitLinkage(linkage),
    );
  return appendInstructionSequences(
    testCode,
    makeSequence(["val"], [], [test("is-true", register("val")), branch(trueLabel)]),
    alternative,
    makeSequence([], [], [gotoLabel(afterLabel), { tag: "label", name: trueLabel }]),
    consequent,
    makeSequence([], [], [{ tag: "label", name: afterLabel }]),
  );
};

const compileLambdaParts = (
  params: ReadonlyArray<{ kind: string; name: string }>,
  body: Block,
  context: CompileContext,
): { readonly build: InstructionSequence; readonly bodySeq: InstructionSequence } | GuestError => {
  const entry = makeLabel("entry");
  const after = makeLabel("after-lambda");
  const compiledBody = compileSequence(body.body, returnLinkage, context);
  if (isCompileError(compiledBody)) {
    return compiledBody;
  }
  const names = params.filter((param) => param.kind !== "rest").map((param) => param.name);
  const restName = params.find((param) => param.kind === "rest")?.name ?? "";
  const build = makeSequence(
    ["env"],
    ["val"],
    [
      assign(
        "val",
        op(
          "make-procedure",
          constant(entry),
          constant(names as Word),
          register("env"),
          constant(restName),
        ),
      ),
    ],
  );
  const bodySeq = appendInstructionSequences(
    makeSequence([], [], [gotoLabel(after), { tag: "label", name: entry }]),
    compiledBody,
    makeSequence([], [], [{ tag: "label", name: after }]),
  );
  return { build, bodySeq };
};

const compileLambda = (
  params: Block extends never ? never : ReadonlyArray<{ kind: string; name: string }>,
  body: Block,
  linkage: Linkage,
  context: CompileContext,
): CompileResult => {
  const parts = compileLambdaParts(params, body, context);
  if (!("build" in parts)) {
    return parts;
  }
  return appendInstructionSequences(parts.bodySeq, parts.build, emitLinkage(linkage));
};

const compileApplication = (
  expr: Extract<Expr, { tag: "call" }>,
  linkage: Linkage,
  context: CompileContext,
): CompileResult => {
  if (isConsoleLog(expr)) {
    const argument = expr.args[0]?.expr;
    if (argument === undefined || expr.args.length !== 1) {
      return compileError("compiled/console.log");
    }
    const value = compileExpression(argument, "val", nextLinkage, context);
    if (isCompileError(value)) {
      return value;
    }
    return withLinkage(
      appendInstructionSequences(
        value,
        makeSequence(
          ["val"],
          ["val"],
          [perform("print-value", register("val")), assign("val", constant(undefined))],
        ),
      ),
      linkage,
    );
  }
  const operator = compileExpression(expr.callee, "val", nextLinkage, context);
  if (isCompileError(operator)) {
    return operator;
  }
  const toProc = makeSequence(["val"], ["proc"], [save("proc"), assign("proc", register("val"))]);
  const operands = compileOperandList(expr.args, context);
  if (isCompileError(operands)) {
    return operands;
  }
  const afterCompound = makeLabel("after-call");
  const afterPrimitive = makeLabel("after-primitive");
  const afterShared = makeLabel("after-shared");
  const primitive = makeLabel("primitive-apply");
  const compound = makeLabel("compound-apply");
  const raised = makeLabel("call-raised");
  const pending = makeLabel("call-pending");
  const resume = makeLabel("call-resume");
  const dispatch = makeSequence(
    ["proc"],
    ["proc"],
    [
      test("is-primitive-procedure", register("proc")),
      branch(primitive),
      gotoLabel(compound),
      { tag: "label", name: primitive },
      assign("val", op("apply-primitive-procedure", register("proc"), register("argl"))),
      test("is-error-value", register("val")),
      branch(raised),
      gotoLabel(afterPrimitive),
      { tag: "label", name: compound },
      save("continue"),
      save("item"),
      save("env"),
      assign("continue", constant({ tag: "symbol", name: afterCompound })),
      assign(
        "env",
        op(
          "extend-environment",
          op("procedure-parameters", register("proc")),
          register("argl"),
          op("procedure-environment", register("proc")),
          op("procedure-rest", register("proc")),
        ),
      ),
      assign("item", op("procedure-entry", register("proc"))),
      gotoRegister("item"),
      { tag: "label", name: raised },
      assign("thrown", op("throw-box", register("val"))),
      context.labels.throwTarget === null
        ? gotoRegister("continue")
        : gotoLabel(context.labels.throwTarget),
      { tag: "label", name: afterCompound },
      restore("env"),
      restore("item"),
      restore("continue"),
      restore("proc"),
      gotoLabel(afterShared),
      { tag: "label", name: afterPrimitive },
      restore("proc"),
      { tag: "label", name: afterShared },
    ],
  );
  const throwCheck = makeSequence(
    ["thrown"],
    ["thrown"],
    [
      test("throw-pending", register("thrown")),
      branch(pending),
      gotoLabel(resume),
      { tag: "label", name: pending },
      context.labels.throwTarget === null
        ? gotoRegister("continue")
        : gotoLabel(context.labels.throwTarget),
      { tag: "label", name: resume },
    ],
  );
  return appendInstructionSequences(
    operator,
    toProc,
    operands,
    dispatch,
    throwCheck,
    emitLinkage(linkage),
  );
};

/** Compiles one statement under `linkage` (statements leave their value in `val`). */
export const compileStatement = (
  stmt: Stmt | Decl,
  linkage: Linkage,
  context: CompileContext,
): CompileResult => {
  switch (stmt.tag) {
    case "expr-stmt":
      return compileExpression(stmt.expr, "val", linkage, context);
    case "block": {
      const outerBreak = context.labels.breakTarget;
      const outerContinue = context.labels.continueTarget;
      const outerThrow = context.labels.throwTarget;
      const breakLabel = outerBreak === null ? null : makeLabel("block-break");
      const continueLabel = outerContinue === null ? null : makeLabel("block-continue");
      const throwLabel = outerThrow === null ? null : makeLabel("block-throw");
      const inner = compileSequence(stmt.body, nextLinkage, {
        ...context,
        labels: {
          throwTarget: throwLabel ?? outerThrow,
          breakTarget: breakLabel ?? outerBreak,
          continueTarget: continueLabel ?? outerContinue,
          returnTarget: context.labels.returnTarget,
        },
      });
      if (isCompileError(inner)) {
        return inner;
      }
      const afterLabel = makeLabel("block-after");
      return appendInstructionSequences(
        makeSequence(
          ["env"],
          ["env"],
          [assign("env", op("extend-environment", constant([]), constant([]), register("env")))],
        ),
        inner,
        makeSequence(
          ["env"],
          ["env"],
          [
            assign("env", op("parent-environment", register("env"))),
            gotoLabel(afterLabel),
            ...(breakLabel === null || outerBreak === null
              ? []
              : [
                  { tag: "label", name: breakLabel } as MachineStatement,
                  assign("env", op("parent-environment", register("env"))),
                  gotoLabel(outerBreak),
                ]),
            ...(continueLabel === null || outerContinue === null
              ? []
              : [
                  { tag: "label", name: continueLabel } as MachineStatement,
                  assign("env", op("parent-environment", register("env"))),
                  gotoLabel(outerContinue),
                ]),
            ...(throwLabel === null || outerThrow === null
              ? []
              : [
                  { tag: "label", name: throwLabel } as MachineStatement,
                  assign("env", op("parent-environment", register("env"))),
                  gotoLabel(outerThrow),
                ]),
            { tag: "label", name: afterLabel },
          ],
        ),
        emitLinkage(linkage),
      );
    }
    case "var-decl":
    case "function-decl":
      return compileDeclaration(stmt, linkage, context);
    case "if": {
      const testCode = compileExpression(stmt.test, "val", nextLinkage, context);
      if (isCompileError(testCode)) {
        return testCode;
      }
      const consequent = compileStatement(stmt.consequent, linkage, context);
      if (isCompileError(consequent)) {
        return consequent;
      }
      const alternativeBranch =
        stmt.alternative === null
          ? undefined
          : compileStatement(stmt.alternative, linkage, context);
      if (alternativeBranch !== undefined && isCompileError(alternativeBranch)) {
        return alternativeBranch;
      }
      const trueLabel = makeLabel("if-true");
      const afterLabel = makeLabel("if-after");
      const alternative: InstructionSequence =
        alternativeBranch ??
        appendInstructionSequences(
          makeSequence([], ["val"], [assign("val", constant(undefined))]),
          emitLinkage(linkage),
        );
      return appendInstructionSequences(
        testCode,
        makeSequence(["val"], [], [test("is-true", register("val")), branch(trueLabel)]),
        alternative,
        makeSequence([], [], [gotoLabel(afterLabel), { tag: "label", name: trueLabel }]),
        consequent,
        makeSequence([], [], [{ tag: "label", name: afterLabel }]),
      );
    }
    case "while":
      return compileWhile(stmt, linkage, context);
    case "for-of":
      return compileForOf(stmt, linkage, context);
    case "return": {
      const target = context.labels.returnTarget;
      const retLinkage: Linkage = target === null ? returnLinkage : gotoLinkage(target);
      if (stmt.argument === null) {
        return appendInstructionSequences(
          makeSequence([], ["val"], [assign("val", constant(undefined))]),
          emitLinkage(retLinkage),
        );
      }
      return compileExpression(stmt.argument, "val", retLinkage, context);
    }
    case "throw": {
      const value = compileExpression(stmt.argument, "val", nextLinkage, context);
      if (isCompileError(value)) {
        return value;
      }
      const target = context.labels.throwTarget;
      const raise: MachineStatement[] =
        target === null
          ? [assign("thrown", op("throw-box", register("val"))), gotoRegister("continue")]
          : [assign("thrown", op("throw-box", register("val"))), gotoLabel(target)];
      return appendInstructionSequences(value, makeSequence(["val"], ["thrown"], raise));
    }
    case "break": {
      const target = context.labels.breakTarget;
      return target === null
        ? compileError("compiled/break")
        : makeSequence([], [], [gotoLabel(target)]);
    }
    case "continue": {
      const target = context.labels.continueTarget;
      return target === null
        ? compileError("compiled/continue")
        : makeSequence([], [], [gotoLabel(target)]);
    }
    case "try":
      return compileTry(stmt, linkage, context);
    case "switch":
      return compileSwitch(stmt, linkage, context);
    case "type-decl":
    case "interface-decl":
    case "import":
      return withLinkage(makeSequence([], ["val"], [assign("val", constant(undefined))]), linkage);
    default:
      return compileError(`compiled/statement/${(stmt as { tag: string }).tag}`);
  }
};

const compileFunctionDeclaration = (
  decl: Extract<Decl, { tag: "function-decl" }>,
  context: CompileContext,
): { readonly hoist: InstructionSequence; readonly atDecl: InstructionSequence } | GuestError => {
  const parts = compileLambdaParts(decl.params, decl.body, context);
  if (!("build" in parts)) {
    return parts;
  }
  const define = makeSequence(
    ["val", "env"],
    ["val"],
    [assign("val", op("define-variable", constant(decl.name), register("val"), register("env")))],
  );
  return {
    hoist: appendInstructionSequences(parts.build, define),
    atDecl: appendInstructionSequences(parts.bodySeq, parts.build, define),
  };
};

const compileDeclaration = (
  decl: Decl,
  linkage: Linkage,
  context: CompileContext,
): CompileResult => {
  if (decl.tag === "function-decl") {
    const compiled = compileFunctionDeclaration(decl, context);
    if (!("hoist" in compiled)) {
      return compiled;
    }
    return withLinkage(compiled.atDecl, linkage);
  }
  if (decl.tag === "var-decl") {
    const init =
      decl.init === null
        ? makeSequence([], ["val"], [assign("val", constant(undefined))])
        : compileExpression(decl.init, "val", nextLinkage, context);
    if (isCompileError(init)) {
      return init;
    }
    const define = makeSequence(
      ["val", "env"],
      ["val"],
      [assign("val", op("define-variable", constant(decl.name), register("val"), register("env")))],
    );
    return withLinkage(appendInstructionSequences(init, define), linkage);
  }
  return compileError(`compiled/declaration/${decl.tag}`);
};

const compileHoists = (
  items: ReadonlyArray<Decl | Stmt>,
  context: CompileContext,
):
  | {
      readonly hoists: ReadonlyArray<InstructionSequence>;
      readonly atDecl: ReadonlyMap<Decl | Stmt, InstructionSequence>;
    }
  | GuestError => {
  const hoists: InstructionSequence[] = [];
  const atDecl = new Map<Decl | Stmt, InstructionSequence>();
  for (const item of items) {
    if (item.tag === "var-decl") {
      hoists.push(
        makeSequence(
          ["env"],
          [],
          [
            perform(
              "predeclare-variable",
              constant(item.name),
              constant(item.kind === "let"),
              register("env"),
            ),
          ],
        ),
      );
    }
    if (item.tag === "function-decl") {
      const compiled = compileFunctionDeclaration(item, context);
      if (!("hoist" in compiled)) {
        return compiled;
      }
      hoists.push(compiled.hoist);
      atDecl.set(item, compiled.atDecl);
    }
  }
  return { hoists, atDecl };
};

const compileSequence = (
  items: ReadonlyArray<Decl | Stmt>,
  linkage: Linkage,
  context: CompileContext,
  shared?: {
    readonly hoists: ReadonlyArray<InstructionSequence>;
    readonly atDecl: ReadonlyMap<Decl | Stmt, InstructionSequence>;
  },
): CompileResult => {
  if (items.length === 0) {
    return withLinkage(makeSequence([], ["val"], [assign("val", constant(undefined))]), linkage);
  }
  const hoisted = shared ?? compileHoists(items, context);
  if (!("hoists" in hoisted)) {
    return hoisted;
  }
  const compiled: InstructionSequence[] = [];
  for (const [index, item] of items.entries()) {
    const link = index === items.length - 1 ? linkage : nextLinkage;
    const atDecl = hoisted.atDecl.get(item);
    const piece =
      atDecl !== undefined
        ? appendInstructionSequences(atDecl, emitLinkage(link))
        : compileStatement(item, link, context);
    if (isCompileError(piece)) {
      return piece;
    }
    compiled.push(piece);
  }
  return appendInstructionSequences(...(shared === undefined ? hoisted.hoists : []), ...compiled);
};

const compileWhile = (
  stmt: Extract<Stmt, { tag: "while" }>,
  linkage: Linkage,
  context: CompileContext,
): CompileResult => {
  const testLabel = makeLabel("while-test");
  const bodyLabel = makeLabel("while-body");
  const endLabel = makeLabel("while-end");
  const testCode = compileExpression(stmt.test, "val", nextLinkage, context);
  if (isCompileError(testCode)) {
    return testCode;
  }
  const body = compileStatement(stmt.body, nextLinkage, {
    ...context,
    labels: { ...context.labels, breakTarget: endLabel, continueTarget: testLabel },
  });
  if (isCompileError(body)) {
    return body;
  }
  return appendInstructionSequences(
    makeSequence([], [], [{ tag: "label", name: testLabel }]),
    testCode,
    makeSequence(
      ["val"],
      [],
      [
        test("is-true", register("val")),
        branch(bodyLabel),
        gotoLabel(endLabel),
        { tag: "label", name: bodyLabel },
      ],
    ),
    body,
    makeSequence(
      [],
      [],
      [gotoLabel(testLabel), { tag: "label", name: endLabel }, assign("val", constant(undefined))],
    ),
    emitLinkage(linkage),
  );
};

const compileForOf = (
  stmt: Extract<Stmt, { tag: "for-of" }>,
  linkage: Linkage,
  context: CompileContext,
): CompileResult => {
  const iterable = compileExpression(stmt.iterable, "val", nextLinkage, context);
  if (isCompileError(iterable)) {
    return iterable;
  }
  const testLabel = makeLabel("for-test");
  const bodyLabel = makeLabel("for-body");
  const endLabel = makeLabel("for-end");
  // The array and cursor live in two hidden bindings (label-style names no
  // guest identifier can spell), not in registers: the loop body may run any
  // statement, including another loop or a switch, and a `return` from the
  // body leaves through `continue` with the caller restoring `env`.
  const itemsName = makeLabel("for-items");
  const indexName = makeLabel("for-index");
  const lookup = (name: string) => op("lookup-variable-value", constant(name), register("env"));
  const setup = makeSequence(
    ["val", "env"],
    ["val", "env"],
    [
      assign(
        "env",
        op("extend-environment", constant(itemsName), register("val"), register("env")),
      ),
      assign("env", op("extend-environment", constant(indexName), constant(0), register("env"))),
      gotoLabel(testLabel),
      { tag: "label", name: testLabel },
      assign("val", op("array-length", lookup(itemsName))),
      test("number-less", lookup(indexName), register("val")),
      branch(bodyLabel),
      gotoLabel(endLabel),
    ],
  );
  const advanceLabel = makeLabel("for-advance");
  const breakRestore = makeLabel("for-break-restore");
  const bind = makeSequence(
    ["env"],
    ["val", "env"],
    [
      { tag: "label", name: bodyLabel },
      assign("val", op("array-index", lookup(itemsName), lookup(indexName))),
      assign(
        "env",
        op("extend-environment", constant(stmt.name), register("val"), register("env")),
      ),
    ],
  );
  const advance = makeSequence(
    ["env"],
    ["val", "env"],
    [
      gotoLabel(advanceLabel),
      { tag: "label", name: advanceLabel },
      assign("env", op("parent-environment", register("env"))),
      assign(
        "val",
        op(
          "set-variable-value",
          constant(indexName),
          op("number-add", lookup(indexName), constant(1)),
          register("env"),
        ),
      ),
      gotoLabel(testLabel),
      { tag: "label", name: breakRestore },
      assign("env", op("parent-environment", register("env"))),
      gotoLabel(endLabel),
      { tag: "label", name: endLabel },
      assign("env", op("parent-environment", op("parent-environment", register("env")))),
    ],
  );
  const loopBody = compileStatement(stmt.body, nextLinkage, {
    ...context,
    labels: { ...context.labels, breakTarget: breakRestore, continueTarget: advanceLabel },
  });
  if (isCompileError(loopBody)) {
    return loopBody;
  }
  return appendInstructionSequences(
    iterable,
    setup,
    bind,
    loopBody,
    advance,
    makeSequence([], ["val"], [assign("val", constant(undefined))]),
    emitLinkage(linkage),
  );
};

const compileTry = (
  stmt: Extract<Stmt, { tag: "try" }>,
  linkage: Linkage,
  context: CompileContext,
): CompileResult => {
  const outerBreak = context.labels.breakTarget;
  const outerContinue = context.labels.continueTarget;
  const outerThrow = context.labels.throwTarget;
  const returnCapture = makeLabel("try-return");
  const breakCapture = outerBreak === null ? null : makeLabel("try-break");
  const continueCapture = outerContinue === null ? null : makeLabel("try-continue");
  const throwCapture = makeLabel("try-throw");
  const catchLabel = stmt.handler === null ? throwCapture : makeLabel("catch");
  const finallyLabel = makeLabel("finally");
  const dispatchReturn = makeLabel("try-dispatch-return");
  const dispatchBreak = breakCapture === null ? null : makeLabel("try-dispatch-break");
  const dispatchContinue = continueCapture === null ? null : makeLabel("try-dispatch-continue");
  const dispatchThrow = makeLabel("try-dispatch-throw");
  const finBreak = outerBreak === null ? null : makeLabel("finally-break");
  const finContinue = outerContinue === null ? null : makeLabel("finally-continue");
  const finReturn = makeLabel("finally-return");
  const finThrow = makeLabel("finally-throw");
  const endLabel = makeLabel("try-end");
  const captures: typeof context.labels = {
    throwTarget: throwCapture,
    breakTarget: breakCapture,
    continueTarget: continueCapture,
    returnTarget: returnCapture,
  };
  const body = compileSequence(stmt.block.body, gotoLinkage(finallyLabel), {
    ...context,
    labels: { ...captures, throwTarget: catchLabel },
  });
  if (isCompileError(body)) {
    return body;
  }
  let handler: InstructionSequence = emptySequence;
  if (stmt.handler !== null) {
    const handlerBody = compileSequence(stmt.handler.body.body, gotoLinkage(finallyLabel), {
      ...context,
      labels: captures,
    });
    if (isCompileError(handlerBody)) {
      return handlerBody;
    }
    handler = appendInstructionSequences(
      makeSequence(
        ["thrown", "env", "val"],
        ["thrown", "env", "val"],
        [
          { tag: "label", name: catchLabel },
          test("throw-error-pending", register("thrown")),
          branch(throwCapture),
          assign("val", op("throw-value", register("thrown"))),
          assign("thrown", op("throw-clear")),
          assign(
            "env",
            op(
              "extend-environment",
              constant(stmt.handler.param ?? "__thrown"),
              register("val"),
              op("parent-environment", register("env")),
            ),
          ),
        ],
      ),
      handlerBody,
    );
  }
  const finalizer =
    stmt.finalizer === null
      ? emptySequence
      : compileSequence(stmt.finalizer.body, nextLinkage, {
          ...context,
          labels: {
            throwTarget: finThrow,
            breakTarget: finBreak,
            continueTarget: finContinue,
            returnTarget: finReturn,
          },
        });
  if (isCompileError(finalizer)) {
    return finalizer;
  }
  return appendInstructionSequences(
    makeSequence(
      ["env", "continue", "pending"],
      ["env", "continue", "pending"],
      [
        save("pending"),
        save("env"),
        assign("env", op("extend-environment", constant([]), constant([]), register("env"))),
        save("continue"),
        assign("continue", constant({ tag: "symbol", name: returnCapture })),
        assign("pending", constant(undefined)),
      ],
    ),
    body,
    makeSequence(
      ["val", "pending"],
      ["pending"],
      [
        { tag: "label", name: returnCapture },
        assign("pending", op("pending-mark", constant("return"), register("val"))),
        gotoLabel(finallyLabel),
        ...(breakCapture === null
          ? []
          : [
              { tag: "label", name: breakCapture } as MachineStatement,
              assign("pending", op("pending-mark", constant("break"), constant(undefined))),
              gotoLabel(finallyLabel),
            ]),
        ...(continueCapture === null
          ? []
          : [
              { tag: "label", name: continueCapture } as MachineStatement,
              assign("pending", op("pending-mark", constant("continue"), constant(undefined))),
              gotoLabel(finallyLabel),
            ]),
        { tag: "label", name: throwCapture },
        assign("pending", op("pending-mark", constant("throw"), constant(undefined))),
        gotoLabel(finallyLabel),
      ],
    ),
    handler,
    makeSequence(
      ["env", "continue"],
      ["env", "continue"],
      [
        { tag: "label", name: finallyLabel },
        restore("continue"),
        restore("env"),
        assign("env", op("extend-environment", constant([]), constant([]), register("env"))),
      ],
    ),
    finalizer,
    makeSequence(
      ["env", "pending", "thrown", "val", "continue"],
      ["val", "env", "pending", "thrown"],
      [
        assign("env", op("parent-environment", register("env"))),
        test("pending-is", register("pending"), constant("return")),
        branch(dispatchReturn),
        ...(dispatchBreak === null
          ? []
          : [test("pending-is", register("pending"), constant("break")), branch(dispatchBreak)]),
        ...(dispatchContinue === null
          ? []
          : [
              test("pending-is", register("pending"), constant("continue")),
              branch(dispatchContinue),
            ]),
        test("pending-is", register("pending"), constant("throw")),
        branch(dispatchThrow),
        gotoLabel(endLabel),
        { tag: "label", name: dispatchReturn },
        assign("val", op("pending-value", register("pending"))),
        restore("pending"),
        gotoRegister("continue"),
        ...(dispatchBreak === null || outerBreak === null
          ? []
          : [
              { tag: "label", name: dispatchBreak } as MachineStatement,
              restore("pending"),
              gotoLabel(outerBreak),
            ]),
        ...(dispatchContinue === null || outerContinue === null
          ? []
          : [
              { tag: "label", name: dispatchContinue } as MachineStatement,
              restore("pending"),
              gotoLabel(outerContinue),
            ]),
        { tag: "label", name: dispatchThrow },
        restore("pending"),
        test("throw-error-pending", register("thrown")),
        branch("uncaught-throw"),
        outerThrow === null ? gotoRegister("continue") : gotoLabel(outerThrow),
        ...(finBreak === null || outerBreak === null
          ? []
          : [
              { tag: "label", name: finBreak } as MachineStatement,
              assign("env", op("parent-environment", register("env"))),
              restore("pending"),
              gotoLabel(outerBreak),
            ]),
        ...(finContinue === null || outerContinue === null
          ? []
          : [
              { tag: "label", name: finContinue } as MachineStatement,
              assign("env", op("parent-environment", register("env"))),
              restore("pending"),
              gotoLabel(outerContinue),
            ]),
        { tag: "label", name: finReturn },
        assign("env", op("parent-environment", register("env"))),
        restore("pending"),
        gotoRegister("continue"),
        { tag: "label", name: finThrow },
        assign("env", op("parent-environment", register("env"))),
        restore("pending"),
        outerThrow === null ? gotoRegister("continue") : gotoLabel(outerThrow),
        { tag: "label", name: endLabel },
        restore("pending"),
        assign("val", constant(undefined)),
      ],
    ),
    emitLinkage(linkage),
  );
};

const compileSwitch = (
  stmt: Extract<Stmt, { tag: "switch" }>,
  linkage: Linkage,
  context: CompileContext,
): CompileResult => {
  const discriminant = compileExpression(stmt.discriminant, "val", nextLinkage, context);
  if (isCompileError(discriminant)) {
    return discriminant;
  }
  const endLabel = makeLabel("switch-end");
  const throwLabel = makeLabel("switch-throw");
  const caseLabels = stmt.cases.map(() => makeLabel("case"));
  const bodyLabels = stmt.cases.map(() => makeLabel("case-body"));
  const defaultLabel = makeLabel("default");
  const defaultBodyLabel = makeLabel("default-body");
  const inner: CompileContext = {
    ...context,
    labels: { ...context.labels, breakTarget: endLabel, throwTarget: throwLabel },
  };
  const allItems: Array<Decl | Stmt> = [
    ...stmt.cases.flatMap((clause) => [...clause.body]),
    ...(stmt.defaultBody ?? []),
  ];
  const shared = compileHoists(allItems, context);
  if (!("hoists" in shared)) {
    return shared;
  }
  const toDiscriminant = makeSequence(["val"], ["item"], [assign("item", register("val"))]);
  const scan: InstructionSequence[] = [];
  for (const [index, clause] of stmt.cases.entries()) {
    const testCode = compileExpression(clause.test, "val", nextLinkage, inner);
    if (isCompileError(testCode)) {
      return testCode;
    }
    scan.push(
      appendInstructionSequences(
        testCode,
        makeSequence(
          ["val", "item"],
          [],
          [
            assign("val", op("value-equal", register("item"), register("val"))),
            test("is-true", register("val")),
            branch(caseLabels[index] ?? defaultLabel),
          ],
        ),
      ),
    );
  }
  scan.push(makeSequence([], [], [gotoLabel(defaultLabel)]));
  const scopeEnter = makeSequence(
    ["env"],
    ["env"],
    [assign("env", op("extend-environment", constant([]), constant([]), register("env")))],
  );
  const bodies: InstructionSequence[] = [];
  for (const [index, clause] of stmt.cases.entries()) {
    const nextLabel = bodyLabels[index + 1] ?? defaultBodyLabel;
    const body = compileSequence(clause.body, gotoLinkage(nextLabel), inner, shared);
    if (isCompileError(body)) {
      return body;
    }
    bodies.push(
      makeSequence([], [], [{ tag: "label", name: caseLabels[index] ?? defaultLabel }]),
      ...shared.hoists,
      makeSequence([], [], [gotoLabel(bodyLabels[index] ?? defaultBodyLabel)]),
      makeSequence([], [], [{ tag: "label", name: bodyLabels[index] ?? defaultBodyLabel }]),
      body,
    );
  }
  const defaultBody = compileSequence(stmt.defaultBody ?? [], gotoLinkage(endLabel), inner, shared);
  if (isCompileError(defaultBody)) {
    return defaultBody;
  }
  bodies.push(
    makeSequence([], [], [{ tag: "label", name: defaultLabel }]),
    ...shared.hoists,
    makeSequence([], [], [gotoLabel(defaultBodyLabel)]),
    makeSequence([], [], [{ tag: "label", name: defaultBodyLabel }]),
    defaultBody,
  );
  return appendInstructionSequences(
    discriminant,
    toDiscriminant,
    scopeEnter,
    ...scan,
    ...bodies,
    makeSequence(
      ["env"],
      ["env"],
      [
        { tag: "label", name: throwLabel },
        assign("env", op("parent-environment", register("env"))),
        context.labels.throwTarget === null
          ? gotoRegister("continue")
          : gotoLabel(context.labels.throwTarget),
      ],
    ),
    makeSequence(
      ["env"],
      ["env", "val"],
      [
        { tag: "label", name: endLabel },
        assign("env", op("parent-environment", register("env"))),
        assign("val", constant(undefined)),
      ],
    ),
    emitLinkage(linkage),
  );
};

/** The book's `compile`: one top-level form to an instruction sequence. */
export const compileDeclarationOrStatement = (
  item: Decl | Stmt,
  context: CompileContext,
): CompileResult => compileStatement(item, nextLinkage, context);

/** Compiles a whole admitted program to one instruction stream. */
export const compileProgram = (
  program: ReadonlyArray<Decl | Stmt>,
): CompiledProgram | GuestError => {
  const context: CompileContext = {
    output: [],
    labels: {
      throwTarget: null,
      breakTarget: null,
      continueTarget: null,
      returnTarget: null,
    },
  };
  const body = compileSequence(program, nextLinkage, context);
  if (isCompileError(body)) {
    return body;
  }
  const sequences: InstructionSequence[] = [
    body,
    makeSequence([], [], [{ tag: "label", name: "uncaught-throw" }, gotoLabel("program-end")]),
    makeSequence([], [], [{ tag: "label", name: "program-end" }]),
  ];
  const instructions = sequences.flatMap((sequence) => [...sequence.statements]);
  return { sequences, instructions };
};

/** Compiles and runs one admitted unit on the teaching machine. */
export const compileAndRun = (source: string, modules: LinkedModules = {}): RunResult => {
  const admission = admitSource(source);
  if (!admission.ok) {
    return {
      outcome: fail({
        tag: "unknown-syntax",
        construct:
          admission.diagnostics[0]?.construct ?? `TS${admission.hostDiagnostics[0]?.code ?? 0}`,
      }),
      transcript: [],
    };
  }
  const compiled = compileProgram(admission.program);
  if (isCompileError(compiled)) {
    return { outcome: fail(compiled), transcript: [] };
  }
  const output: string[] = [];
  const machine: Machine<Word> = makeMachine<Word>({
    registers: ["val", "env", "argl", "proc", "continue", "entry", "thrown", "item", "pending"],
    operations: compiledOperations(output),
    controller: compiled.instructions,
  });
  const session = new Session("core", modules);
  const environment = session.globalEnv();
  for (const form of admission.program) {
    // Imports link before the first instruction runs, like module instantiation.
    const error = form.tag === "import" ? session.linkImport(form, environment) : null;
    if (error !== null) {
      return { outcome: fail(error), transcript: [] };
    }
  }
  machine.writeRegister("env", environment);
  machine.writeRegister("thrown", undefined);
  machine.writeRegister("continue", { tag: "symbol", name: "program-end" });
  const run = machine.run(1_000_000);
  if (run.error !== null) {
    const detail =
      run.error.tag === "unknown-operation" || run.error.tag === "unknown-register"
        ? run.error.name
        : run.error.tag;
    return {
      outcome: fail({ tag: "unknown-syntax", construct: `machine-error/${detail}` }),
      transcript: output,
    };
  }
  const thrown = machine.readRegister("thrown");
  const pending =
    typeof thrown === "object" && thrown !== null && "kind" in thrown
      ? (thrown as Transfer)
      : undefined;
  if (pending?.kind === "error") {
    return { outcome: fail(pending.error), transcript: output };
  }
  if (pending?.kind === "throw") {
    return { outcome: fail({ tag: "guest-throw", value: pending.value }), transcript: output };
  }
  return { outcome: ok(machine.readRegister("val") as Value), transcript: output };
};

/** Compiles one expression and returns its instruction sequence (exercise seam). */
export const compileSelfEvaluating = (value: number | string | boolean | null): CompileResult =>
  makeSequence([], ["val"], [assign("val", constant(value as Word))]);

/** Compiles a variable read (exercise seam). */
export const compileVariable = (name: string): CompileResult =>
  makeSequence(
    ["env"],
    ["val"],
    [assign("val", op("lookup-variable-value", constant(name), register("env")))],
  );

/** Compiles a primitive application (exercise seam). */
export const compilePrimitiveOperation = (
  name: string,
  operands: ReadonlyArray<string>,
): CompileResult =>
  makeSequence(
    operands,
    ["val"],
    [assign("val", op(name, ...operands.map((operand) => register(operand))))],
  );

/** Reports an unsupported construct at compile time (exercise seam). */
export const compileErrorAt = (construct: string): GuestError => compileError(construct);

/** Runs the compiled engine over admitted source (alias of {@link compileAndRun}). */
export const runCompiledSource = compileAndRun;
