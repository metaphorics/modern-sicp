// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 5.4

import {
  CALLBACK_METHODS,
  type LinkedModules,
  type RunResult,
  Session,
} from "@sicp-ts/ch4/01-metacircular";
/**
 * The explicit-control evaluator (host-subsets grammar section 5): the
 * checked host-subset syntax executes on the teaching machine. The typed
 * controller is instruction data — form dispatch, argument accumulation in
 * `argl`, and the `continue` register — and every operation is primitive:
 * environment access, procedure construction, arithmetic, and structure
 * access. No operation evaluates a guest form; evaluation recursion is the
 * controller's recursion, so procedure bodies run on the machine.
 *
 * Non-local control (return, throw, break, continue) travels through the
 * `transfer` register: a construct's continuation restores its frame,
 * forwards an unresolved transfer to its own `continue`, and the loop,
 * switch, try, and application frames intercept the kinds they own.
 */
import { format } from "@sicp-ts/ch4/read";
import { builtinMember } from "@sicp-ts/ch4/runtime/builtins";
import { child, type Env, findCell, makeCell } from "@sicp-ts/ch4/runtime/env";
import { fail, type GuestError, ok } from "@sicp-ts/ch4/runtime/errors";
import {
  ArrayValue,
  Closure,
  ErrorValue,
  MapValue,
  makeMap,
  makePrimitive,
  makeSet,
  PrimitiveProcedure,
  RecordValue,
  SetValue,
  ThunkValue,
  type Value,
} from "@sicp-ts/ch4/runtime/value";
import type { Arg, Block, CaseClause, Decl, Expr, Param, Stmt } from "@sicp-ts/ch4/syntax/ast";
import { admitSource } from "@sicp-ts/ch4/syntax/check";

import {
  type MachineStatement as GenericMachineStatement,
  type Input,
  assign as machineAssign,
  branch as machineBranch,
  constant as machineConstant,
  gotoLabel as machineGotoLabel,
  gotoRegister as machineGotoRegister,
  op as machineOp,
  perform as machinePerform,
  register as machineRegister,
  restore as machineRestore,
  save as machineSave,
  test as machineTest,
  type Operation,
  type OperationCall,
  type Source,
} from "./01-register-machines.ts";
import { Machine, makeMachine } from "./02-simulator.ts";

export { readProgram, readProgram as parse } from "@sicp-ts/ch4/read";

/** A checked evaluator failure carried through machine registers. */
export class MachineErrorValue {
  readonly error: GuestError;

  constructor(error: GuestError) {
    this.error = error;
  }
}

/** A controller-created iterator cursor for `for-of`. */
interface IterationState {
  readonly items: ReadonlyArray<Value>;
  readonly index: number;
}

/** A machine word: shared syntax, runtime values, environments, or transfer data. */
export type Word =
  | Expr
  | Decl
  | Stmt
  | Block
  | Env
  | Value
  | MachineErrorValue
  | Transfer
  | Arg
  | CaseClause
  | Param
  | ReadonlyArray<Word>
  | IterationState
  | { readonly tag: "symbol"; readonly name: string }
  | undefined;

/** Pending non-local control travelling through the transfer register. */
export type Transfer =
  | { readonly kind: "return" }
  | { readonly kind: "throw"; readonly value: Value }
  | { readonly kind: "error"; readonly error: GuestError }
  | { readonly kind: "break" }
  | { readonly kind: "continue-loop" };

type EvaluatorMachineStatement = GenericMachineStatement<Word>;

const assign = (name: string, source: Source<Word>): EvaluatorMachineStatement =>
  machineAssign<Word>(name, source);
const branch = (labelName: string): EvaluatorMachineStatement => machineBranch<Word>(labelName);
const constant = (value: Word): Input<Word> => machineConstant<Word>(value);
const gotoLabel = (labelName: string): EvaluatorMachineStatement =>
  machineGotoLabel<Word>(labelName);
const gotoRegister = (registerName: string): EvaluatorMachineStatement =>
  machineGotoRegister<Word>(registerName);
const op = (name: string, ...args: ReadonlyArray<Source<Word>>): OperationCall<Word> =>
  machineOp<Word>(name, ...args);
const perform = (name: string, ...args: ReadonlyArray<Source<Word>>): EvaluatorMachineStatement =>
  machinePerform<Word>(name, ...args);
const register = (name: string): Input<Word> => machineRegister<Word>(name);
const restore = (name: string): EvaluatorMachineStatement => machineRestore<Word>(name);
const save = (name: string): EvaluatorMachineStatement => machineSave<Word>(name);
const test = (name: string, ...args: ReadonlyArray<Source<Word>>): EvaluatorMachineStatement =>
  machineTest<Word>(name, ...args);

type Form = Expr | Decl | Stmt;

const isForm = (word: Word): word is Form =>
  typeof word === "object" &&
  word !== null &&
  "tag" in word &&
  "span" in word &&
  typeof word.tag === "string";

const isEnv = (word: Word): word is Env =>
  typeof word === "object" && word !== null && "bindings" in word && "parent" in word;

const formOf = (word: Word): Form | undefined => (isForm(word) && !isEnv(word) ? word : undefined);
const envOf = (word: Word): Env | undefined => (isEnv(word) ? word : undefined);

const isValue = (word: Word): word is Value =>
  word === null ||
  typeof word !== "object" ||
  word instanceof Closure ||
  word instanceof PrimitiveProcedure ||
  word instanceof ArrayValue ||
  word instanceof RecordValue ||
  word instanceof MapValue ||
  word instanceof SetValue ||
  word instanceof ThunkValue ||
  word instanceof ErrorValue;

const listOf = (word: Word): readonly Word[] => (Array.isArray(word) ? word : []);

const splitParamNames = (params: readonly unknown[]): { params: string[]; rest: string | null } => {
  const names: string[] = [];
  let rest: string | null = null;
  for (const param of params) {
    const entry = param as { kind?: unknown; name?: unknown };
    const name = typeof entry.name === "string" ? entry.name : "";
    if (entry.kind === "rest") {
      rest = name;
    } else {
      names.push(name);
    }
  }
  return { params: names, rest };
};

const predeclare = (forms: readonly Word[], env: Env | null): void => {
  if (env === null) {
    return;
  }
  for (const item of forms) {
    const node = formOf(item);
    if (node === undefined) {
      continue;
    }
    if (node.tag === "var-decl") {
      env.bindings.set(node.name, makeCell(undefined, false, node.kind === "let"));
    }
    if (node.tag === "function-decl") {
      const { params, rest } = splitParamNames(node.params);
      const closure = new Closure(params, rest, node.body, env);
      env.bindings.set(node.name, makeCell(closure, true, false));
    }
  }
};

const errorOf = (word: Word): MachineErrorValue | undefined =>
  word instanceof MachineErrorValue ? word : undefined;
const isContinuation = (word: Word, name: string): boolean =>
  typeof word === "object" &&
  word !== null &&
  "tag" in word &&
  word.tag === "symbol" &&
  "name" in word &&
  word.name === name;

interface EvaluatorState {
  readonly session: Session;
  forms: ReadonlyArray<Decl | Stmt>;
  next: number;
  values: Value[];
}

/** One explicit-control evaluator over admitted source. */
export interface Evaluator {
  readonly machine: Machine<Word>;
  run(): RunResult;
}

// ---------------------------------------------------------------------
// Primitive operations: no operation evaluates a guest form.
// ---------------------------------------------------------------------

const operationsFor = (state: EvaluatorState): Readonly<Record<string, Operation<Word>>> => ({
  nextForm: () => {
    const item = state.forms[state.next];
    state.next += 1;
    return item;
  },
  isDone: (args) => args[0] === undefined,
  isApplyBodyContinuation: (args) => isContinuation(args[0], "apply-body-done"),
  isTailReturnContinuation: (args) => isContinuation(args[0], "tail-return"),
  isTailCall: (args) => args[0] === true,
  literalValue: (args) => {
    const node = formOf(args[0]);
    if (
      node === undefined ||
      !["number", "string", "boolean", "null", "undefined"].includes(node.tag)
    ) {
      return undefined;
    }
    if (node.tag === "number" || node.tag === "string" || node.tag === "boolean") {
      return node.value;
    }
    return node.tag === "null" ? null : undefined;
  },
  variableName: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "name" in node && typeof node.name === "string" ? node.name : "";
  },
  lookupVariableValue: (args) => {
    const name = typeof args[0] === "string" ? args[0] : "";
    const env = envOf(args[1]) ?? null;
    const cell = findCell(env, name);
    if (cell === undefined) {
      return new MachineErrorValue({ tag: "unbound-name", name });
    }
    if (!cell.initialized) {
      return new MachineErrorValue({ tag: "tdz-access", name });
    }
    return cell.value;
  },
  setVariableValue: (args) => {
    const name = typeof args[0] === "string" ? args[0] : "";
    const cell = findCell(envOf(args[2]) ?? null, name);
    if (cell === undefined) {
      return new MachineErrorValue({ tag: "unbound-name", name });
    }
    if (!cell.initialized) {
      return new MachineErrorValue({ tag: "tdz-access", name });
    }
    if (!cell.mutable) {
      return new MachineErrorValue({
        tag: "bad-operand",
        operator: "=",
        detail: "assignment to a const binding",
      });
    }
    cell.value = args[1] as Value;
    return args[1];
  },
  declareVariable: (args) => {
    const kind = args[0] === "const" ? "const" : "let";
    const name = typeof args[1] === "string" ? args[1] : "";
    const env = envOf(args[3]) ?? null;
    if (env === null) {
      return new MachineErrorValue({ tag: "unbound-name", name });
    }
    const existing = env.bindings.get(name);
    if (existing !== undefined) {
      existing.value = args[2] as Value;
      existing.initialized = true;
    } else {
      env.bindings.set(name, makeCell(args[2] as Value, true, kind === "let"));
    }
    return args[2];
  },
  defineFunction: (args) => {
    const node = formOf(args[0]);
    const env = envOf(args[1]) ?? null;
    if (node === undefined || node.tag !== "function-decl" || env === null) {
      return new MachineErrorValue({ tag: "unknown-syntax", construct: "function-decl" });
    }
    const { params, rest } = splitParamNames(node.params);
    const closure = new Closure(params, rest, node.body, env);
    env.bindings.set(node.name, makeCell(closure, true, false));
    return undefined;
  },
  makeProcedure: (args) => {
    const body = args[1];
    const env = envOf(args[2]);
    if (body === undefined || env === undefined) {
      return new MachineErrorValue({ tag: "unknown-syntax", construct: "arrow" });
    }
    const { params, rest } = splitParamNames(listOf(args[0]));
    return new Closure(params, rest, body as Block, env);
  },
  lambdaParams: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "params" in node ? (node.params as Word) : undefined;
  },
  lambdaBody: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "body" in node ? (node.body as Word) : undefined;
  },
  procedureParams: (args) => {
    const proc = args[0];
    return proc instanceof Closure ? (proc.params as Word) : undefined;
  },
  procedureRest: (args) => {
    const proc = args[0];
    return proc instanceof Closure ? (proc.rest ?? "") : "";
  },
  procedureBody: (args) => {
    const proc = args[0];
    return proc instanceof Closure ? proc.body.body : undefined;
  },
  procedureEnv: (args) => {
    const proc = args[0];
    return proc instanceof Closure ? proc.env : undefined;
  },
  isPrimitiveProcedure: (args) => args[0] instanceof PrimitiveProcedure,
  isClosure: (args) => args[0] instanceof Closure,
  unknownProcedureValue: (args) => {
    const proc = args[0];
    return new MachineErrorValue({
      tag: "not-callable",
      detail:
        typeof proc === "object" && proc !== null && "tag" in proc ? String(proc.tag) : typeof proc,
    });
  },
  applyPrimitiveProcedure: (args) => {
    const proc = args[0];
    const values = Array.isArray(args[1]) ? (args[1] as Value[]) : [];
    if (!(proc instanceof PrimitiveProcedure)) {
      return new MachineErrorValue({ tag: "not-callable", detail: "unknown" });
    }
    const outcome = proc.fn(values as Value[]);
    return outcome.tag === "error" ? new MachineErrorValue(outcome.error) : outcome.value;
  },
  extendEnvironment: (args) => {
    const rawParams = args[0];
    const rawValues = args[1];
    const params = typeof rawParams === "string" ? [rawParams] : listOf(rawParams);
    const values = Array.isArray(rawValues) ? rawValues : [rawValues];
    const parent = envOf(args[2]) ?? null;
    const rest = typeof args[3] === "string" ? args[3] : "";
    const frame = child(parent);
    for (const [index, param] of params.entries()) {
      const name = typeof param === "string" ? param : "";
      frame.bindings.set(name, makeCell(values[index] as Value, true));
    }
    if (rest !== "") {
      frame.bindings.set(
        rest,
        makeCell(new ArrayValue(values.slice(params.length) as Value[]), true),
      );
    }
    return frame;
  },
  predeclareProgram: (args) => {
    predeclare(listOf(args[0]), envOf(args[1]) ?? null);
    return undefined;
  },
  predeclareForms: (args) => {
    predeclare(listOf(args[0]), envOf(args[1]) ?? null);
    return undefined;
  },
  emptyArgList: () => [],
  argExprs: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "args" in node ? (node.args as Word) : undefined;
  },
  arrayElements: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "elements" in node ? (node.elements as Word) : undefined;
  },
  objectKeys: (args) => {
    const node = formOf(args[0]);
    if (node === undefined || !("fields" in node)) {
      return undefined;
    }
    return (node.fields as ReadonlyArray<{ key: string }>).map((field) => field.key);
  },
  objectValues: (args) => {
    const node = formOf(args[0]);
    if (node === undefined || !("fields" in node)) {
      return undefined;
    }
    return (node.fields as ReadonlyArray<{ value: Expr }>).map((field) => ({
      kind: "item",
      expr: field.value,
    }));
  },
  templateChunks: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "chunks" in node ? (node.chunks as Word) : undefined;
  },
  templateExprs: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "exprs" in node ? (node.exprs as Word) : undefined;
  },
  firstOperand: (args) => {
    const operands = listOf(args[0]);
    const first = operands[0];
    if (first === undefined) {
      return undefined;
    }
    const wrapped =
      typeof first === "object" && first !== null && "kind" in first && "expr" in first;
    return wrapped ? ((first as Arg).expr as Word) : (first as Word);
  },
  restOperands: (args) => listOf(args[0]).slice(1),
  noOperands: (args) => listOf(args[0]).length === 0,
  adjoinArg: (args) => {
    const collected = [...listOf(args[0])];
    const operands = listOf(args[1]);
    const first = operands[0];
    const spread =
      typeof first === "object" && first !== null && "kind" in first && first.kind === "spread";
    const value = args[2];
    if (spread && value instanceof ArrayValue) {
      collected.push(...value.items);
    } else {
      collected.push(value);
    }
    return collected;
  },
  calleeOf: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "callee" in node ? (node.callee as Word) : undefined;
  },
  memberObject: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "object" in node ? (node.object as Word) : undefined;
  },
  memberName: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "name" in node && typeof node.name === "string" ? node.name : "";
  },
  memberGet: (args) => {
    const object = args[0];
    const name = typeof args[1] === "string" ? args[1] : "";
    if (object instanceof ArrayValue && name === "length") {
      return object.items.length;
    }
    if (object instanceof MapValue && name === "size") {
      return object.entries.size;
    }
    if (object instanceof SetValue && name === "size") {
      return object.items.size;
    }
    if (object instanceof ErrorValue && name === "message") {
      return object.message;
    }
    if (object instanceof RecordValue && object.fields.has(name)) {
      return object.fields.get(name);
    }
    if (typeof object === "string" && name === "length") {
      return object.length;
    }
    if (object instanceof ArrayValue && CALLBACK_METHODS[name] === true) {
      return makePrimitive(`array.${name}`, (callArgs) =>
        state.session.callArrayCallback(object, name, callArgs),
      );
    }
    if (
      object instanceof ArrayValue ||
      object instanceof MapValue ||
      object instanceof SetValue ||
      typeof object === "string"
    ) {
      const member = builtinMember(object, name);
      if (member !== undefined) {
        return member;
      }
    }
    return new MachineErrorValue({ tag: "unknown-field", field: name });
  },
  memberSet: (args) => {
    const object = args[0];
    const name = typeof args[1] === "string" ? args[1] : "";
    const value = args[2];
    if (!(object instanceof RecordValue)) {
      return new MachineErrorValue({ tag: "unknown-field", field: name });
    }
    if (object.readonlyFields.has(name) && object.fields.has(name)) {
      return new MachineErrorValue({ tag: "readonly-field", field: name });
    }
    object.fields.set(name, value as Value);
    return value;
  },
  indexObject: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "object" in node ? (node.object as Word) : undefined;
  },
  indexIndex: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "index" in node ? (node.index as Word) : undefined;
  },
  indexGet: (args) => {
    const object = args[0];
    const index = args[1];
    if (object instanceof ArrayValue && typeof index === "number") {
      return Number.isInteger(index) && index >= 0 && index < object.items.length
        ? object.items[index]
        : undefined;
    }
    if (typeof object === "string" && typeof index === "number") {
      const characters = [...object];
      return Number.isInteger(index) && index >= 0 && index < characters.length
        ? characters[index]
        : undefined;
    }
    if (object instanceof RecordValue && typeof index === "string") {
      return object.fields.has(index) ? object.fields.get(index) : undefined;
    }
    return new MachineErrorValue({ tag: "bad-operand", operator: "index", detail: String(index) });
  },
  indexSet: (args) => {
    const object = args[0];
    const index = args[1];
    const value = args[2];
    if (object instanceof ArrayValue && typeof index === "number") {
      if (!Number.isInteger(index) || index < 0) {
        return new MachineErrorValue({
          tag: "bad-operand",
          operator: "index",
          detail: String(index),
        });
      }
      while (object.items.length < index) {
        object.items.push(undefined);
      }
      object.items[index] = value as Value;
      return value;
    }
    return new MachineErrorValue({ tag: "bad-operand", operator: "index", detail: String(index) });
  },
  unaryOperator: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "op" in node ? String(node.op) : "";
  },
  binaryOperator: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "op" in node ? String(node.op) : "";
  },
  logicalSkipRight: (args) => {
    const operator = typeof args[0] === "string" ? args[0] : "";
    const value = args[1];
    const truthy = !(
      value === false ||
      value === 0 ||
      value === "" ||
      value === null ||
      value === undefined ||
      (typeof value === "number" && Number.isNaN(value))
    );
    return operator === "&&" ? !truthy : truthy;
  },
  unaryOperand: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "operand" in node ? (node.operand as Word) : undefined;
  },
  unaryValue: (args) => {
    const operator = typeof args[0] === "string" ? args[0] : "";
    const value = args[1];
    if (operator === "!") {
      return (
        value === false ||
        value === 0 ||
        value === "" ||
        value === null ||
        value === undefined ||
        (typeof value === "number" && Number.isNaN(value))
      );
    }
    if (operator === "-") {
      return typeof value === "number"
        ? -value
        : new MachineErrorValue({ tag: "bad-operand", operator, detail: typeof value });
    }
    if (operator === "+") {
      return typeof value === "number"
        ? value
        : new MachineErrorValue({ tag: "bad-operand", operator, detail: typeof value });
    }
    if (operator === "typeof") {
      return value instanceof Closure || value instanceof PrimitiveProcedure
        ? "function"
        : typeof value;
    }
    return typeof value;
  },
  binaryLeft: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "left" in node ? (node.left as Word) : undefined;
  },
  binaryRight: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "right" in node ? (node.right as Word) : undefined;
  },
  binaryValue: (args) => {
    const operator = typeof args[0] === "string" ? args[0] : "";
    const left = args[1];
    const right = args[2];
    if (operator === "+") {
      if (typeof left === "string" || typeof right === "string") {
        return typeof left === "string" || typeof right === "string"
          ? String(renderValue(left as Value) === "null" ? "null" : left) +
              String(renderValue(right as Value) === "null" ? "null" : right)
          : undefined;
      }
      return typeof left === "number" && typeof right === "number"
        ? left + right
        : new MachineErrorValue({ tag: "bad-operand", operator, detail: "mixed-plus" });
    }
    const numbers: Record<string, (l: number, r: number) => number> = {
      "-": (l, r) => l - r,
      "*": (l, r) => l * r,
      "/": (l, r) => l / r,
      "%": (l, r) => l % r,
    };
    const numberOperation = numbers[operator];
    if (operator in numbers && numberOperation !== undefined) {
      return typeof left === "number" && typeof right === "number"
        ? numberOperation(left, right)
        : new MachineErrorValue({ tag: "bad-operand", operator, detail: "non-number" });
    }
    const comparisons: Record<string, (l: number, r: number) => boolean> = {
      "<": (l, r) => l < r,
      "<=": (l, r) => l <= r,
      ">": (l, r) => l > r,
      ">=": (l, r) => l >= r,
    };
    const comparison = comparisons[operator];
    if (operator in comparisons && comparison !== undefined) {
      return typeof left === "number" && typeof right === "number"
        ? comparison(left, right)
        : new MachineErrorValue({ tag: "bad-operand", operator, detail: "non-number" });
    }
    if (operator === "===") {
      return left === right;
    }
    return left !== right;
  },
  truthy: (args) => {
    const value = args[0];
    return !(
      value === false ||
      value === 0 ||
      value === "" ||
      value === null ||
      value === undefined ||
      (typeof value === "number" && Number.isNaN(value))
    );
  },
  conditionalTest: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "test" in node ? (node.test as Word) : undefined;
  },
  conditionalConsequent: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "consequent" in node ? (node.consequent as Word) : undefined;
  },
  conditionalAlternative: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "alternative" in node && node.alternative !== null
      ? (node.alternative as Word)
      : undefined;
  },
  assignTarget: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "target" in node ? (node.target as Word) : undefined;
  },
  assignValue: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "value" in node ? (node.value as Word) : undefined;
  },
  assignTargetName: (args) => {
    const node = formOf(args[0]);
    const target = node !== undefined && "target" in node ? formOf(node.target as Word) : undefined;
    return target !== undefined && "name" in target ? String(target.name) : "";
  },
  exprStmtExpr: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "expr" in node ? (node.expr as Word) : undefined;
  },
  varKindName: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "kind" in node ? String(node.kind) : "let";
  },
  varInit: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "init" in node && node.init !== null
      ? (node.init as Word)
      : undefined;
  },
  blockForms: (args) => {
    const node = formOf(args[0]);
    if (node === undefined) {
      return undefined;
    }
    if (node.tag === "block") {
      return node.body;
    }
    return node.tag === "arrow" ? node.body.body : [node];
  },
  ifConsequent: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "consequent" in node ? (node.consequent as Word) : undefined;
  },
  ifAlternative: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "alternative" in node && node.alternative !== null
      ? (node.alternative as Word)
      : undefined;
  },
  whileBody: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "body" in node ? (node.body as Word) : undefined;
  },
  forOfName: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "name" in node ? String(node.name) : "";
  },
  forOfIterable: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "iterable" in node ? (node.iterable as Word) : undefined;
  },
  forOfBody: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "body" in node ? (node.body as Word) : undefined;
  },
  makeIteration: (args) => {
    const collection = args[0];
    const items: Value[] =
      collection instanceof ArrayValue
        ? [...collection.items]
        : typeof collection === "string"
          ? [...collection].map((character) => character)
          : collection instanceof Set === false &&
              typeof collection === "object" &&
              collection !== null &&
              "items" in collection
            ? [...collection.items]
            : [];
    return { items, index: 0 };
  },
  iterationDone: (args) => {
    const state = args[0] as IterationState | undefined;
    return state === undefined || state.index >= state.items.length;
  },
  iterationItem: (args) => {
    const state = args[0] as IterationState | undefined;
    return state === undefined ? undefined : state.items[state.index];
  },
  iterationNext: (args) => {
    const state = args[0] as IterationState | undefined;
    return state === undefined ? undefined : { items: state.items, index: state.index + 1 };
  },
  switchDiscriminant: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "discriminant" in node ? (node.discriminant as Word) : undefined;
  },
  switchCases: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "cases" in node ? (node.cases as Word) : undefined;
  },
  switchDefaultBody: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && node.tag === "switch"
      ? (node.defaultBody ?? undefined)
      : undefined;
  },
  caseTest: (args) => {
    const cases = listOf(args[0]);
    const first = cases[0];
    return typeof first === "object" && first !== null && "test" in first
      ? ((first as { test: Expr }).test as Word)
      : undefined;
  },
  caseBody: (args) => {
    const cases = listOf(args[0]);
    const first = cases[0];
    return typeof first === "object" && first !== null && "body" in first
      ? ((first as { body: ReadonlyArray<Decl | Stmt> }).body as Word)
      : undefined;
  },
  caseBodiesFrom: (args) => {
    const node = formOf(args[0]);
    const start = typeof args[1] === "number" ? args[1] : 0;
    if (node === undefined || node.tag !== "switch") {
      return [] as Word;
    }
    const forms: Array<Decl | Stmt> = [];
    for (const clause of node.cases.slice(start)) {
      forms.push(...clause.body);
    }
    forms.push(...(node.defaultBody ?? []));
    return forms as Word;
  },
  caseCount: (args) => listOf(args[0]).length,
  restCases: (args) => listOf(args[0]).slice(1),
  noCases: (args) => listOf(args[0]).length === 0,
  tryBlock: (args) => {
    const node = formOf(args[0]);
    if (node === undefined || !("block" in node)) {
      return undefined;
    }
    const block = node.block as Block;
    return { tag: "block", body: block.body, span: block.span } as Word;
  },
  tryHandlerBody: (args) => {
    const node = formOf(args[0]);
    if (node === undefined || !("handler" in node) || node.handler === null) {
      return undefined;
    }
    const block = (node.handler as { body: Block }).body;
    return { tag: "block", body: block.body, span: block.span } as Word;
  },
  tryHandlerParam: (args) => {
    const node = formOf(args[0]);
    if (node === undefined || !("handler" in node) || node.handler === null) {
      return "";
    }
    return (node.handler as { param: string | null }).param ?? "";
  },
  tryFinalizerBody: (args) => {
    const node = formOf(args[0]);
    if (node === undefined || !("finalizer" in node) || node.finalizer === null) {
      return undefined;
    }
    const block = node.finalizer as Block;
    return { tag: "block", body: block.body, span: block.span } as Word;
  },
  hasFinalizer: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "finalizer" in node && node.finalizer !== null;
  },
  hasHandler: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "handler" in node && node.handler !== null;
  },
  returnArgument: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "argument" in node && node.argument !== null
      ? (node.argument as Word)
      : undefined;
  },
  throwArgument: (args) => {
    const node = formOf(args[0]);
    return node !== undefined && "argument" in node ? (node.argument as Word) : undefined;
  },
  arrayValue: (args) => new ArrayValue([...listOf(args[0])] as Value[]),
  objectValue: (args) => {
    const keys = listOf(args[0]);
    const values = listOf(args[1]);
    const fields = new Map<string, Value>();
    for (const [index, key] of keys.entries()) {
      fields.set(String(key), values[index] as Value);
    }
    return new RecordValue(fields, new Set());
  },
  templateValue: (args) => {
    const chunks = listOf(args[0]);
    const values = listOf(args[1]);
    let text = "";
    for (const [index, chunk] of chunks.entries()) {
      text += String(chunk);
      const value = values[index];
      if (value !== undefined && isValue(value)) {
        text += String(renderValue(value));
      }
    }
    return text;
  },
  newErrorValue: (args) => {
    const values = listOf(args[0]);
    return new ErrorValue(values.length > 0 ? renderValue(values[0] as Value) : "");
  },
  newMapValue: (args) => {
    const values = listOf(args[0]);
    const source = values[0];
    if (source === undefined) {
      return makeMap();
    }
    if (!(source instanceof ArrayValue)) {
      return new MachineErrorValue({
        tag: "bad-operand",
        operator: "new Map",
        detail: "entries are not an array",
      });
    }
    const entries: Array<readonly [Value, Value]> = [];
    for (const pair of source.items) {
      if (!(pair instanceof ArrayValue) || pair.items.length !== 2) {
        return new MachineErrorValue({
          tag: "bad-operand",
          operator: "new Map",
          detail: "entry is not a pair",
        });
      }
      entries.push([pair.items[0], pair.items[1]]);
    }
    return makeMap(entries);
  },
  newSetValue: (args) => {
    const values = listOf(args[0]);
    const source = values[0];
    if (source === undefined) {
      return makeSet();
    }
    return source instanceof ArrayValue
      ? makeSet(source.items)
      : new MachineErrorValue({
          tag: "bad-operand",
          operator: "new Set",
          detail: "items are not an array",
        });
  },
  recordOutput: (args) => {
    const values = listOf(args[0]);
    for (const value of values) {
      state.session.transcript.push(renderValue(value as Value));
    }
    return undefined;
  },
  recordResult: (args) => {
    state.values.push(args[0] as Value);
    return args[0];
  },
  throwTransfer: (args) => {
    const error = errorOf(args[0]);
    return error === undefined
      ? ({ kind: "throw", value: args[0] as Value } as Transfer)
      : ({ kind: "error", error: error.error } as Transfer);
  },
  returnTransfer: () => ({ kind: "return" }) as Transfer,
  breakTransfer: () => ({ kind: "break" }) as Transfer,
  loopContinueTransfer: () => ({ kind: "continue-loop" }) as Transfer,
  nullTransfer: () => undefined,
  isTransfer: (args) =>
    args[0] !== undefined && typeof args[0] === "object" && args[0] !== null && "kind" in args[0],
  isReturnTransfer: (args) =>
    typeof args[0] === "object" &&
    args[0] !== null &&
    "kind" in args[0] &&
    args[0].kind === "return",
  isThrowTransfer: (args) =>
    typeof args[0] === "object" &&
    args[0] !== null &&
    "kind" in args[0] &&
    args[0].kind === "throw",
  isBreakTransfer: (args) =>
    typeof args[0] === "object" &&
    args[0] !== null &&
    "kind" in args[0] &&
    args[0].kind === "break",
  isLoopContinueTransfer: (args) =>
    typeof args[0] === "object" &&
    args[0] !== null &&
    "kind" in args[0] &&
    args[0].kind === "continue-loop",
  transferValue: (args) => {
    const transfer = args[0];
    return typeof transfer === "object" && transfer !== null && "value" in transfer
      ? (transfer.value as Word)
      : undefined;
  },
  isErrorValue: (args) => args[0] instanceof MachineErrorValue,
  errorTransfer: (args) => {
    const error = errorOf(args[0]);
    return error === undefined ? undefined : ({ kind: "error", error: error.error } as Transfer);
  },
  unknownFormValue: (args) => {
    const node = formOf(args[0]);
    return new MachineErrorValue({ tag: "unknown-syntax", construct: node?.tag ?? "unknown" });
  },
  literalTag: (args) => {
    const node = formOf(args[0]);
    return node === undefined ? "" : node.tag;
  },
});

/** The guest renderer used for output and errors. */
export const renderValue = (value: Value): string =>
  typeof value === "string" ? value : format(value as never);

// ---------------------------------------------------------------------
// Form predicates: real runtime discrimination over the shared AST.
// ---------------------------------------------------------------------

const tagsOf =
  (...names: string[]): Operation<Word> =>
  (args) => {
    const node = formOf(args[0]);
    return node !== undefined && names.includes(node.tag);
  };

const predicates: Readonly<Record<string, Operation<Word>>> = {
  isLiteral: tagsOf("number", "string", "boolean", "null", "undefined"),
  isTemplate: tagsOf("template"),
  isVariable: tagsOf("variable"),
  isAssign: tagsOf("assign"),
  isConditionalExpr: tagsOf("conditional"),
  isArrow: tagsOf("arrow"),
  isCall: tagsOf("call"),
  isMember: tagsOf("member"),
  isIndex: tagsOf("index"),
  isArray: tagsOf("array"),
  isObject: tagsOf("object"),
  isUnary: tagsOf("unary"),
  isBinary: tagsOf("binary"),
  isLogical: tagsOf("logical"),
  isNewError: tagsOf("new-error"),
  isNewMap: tagsOf("new-map"),
  isNewSet: tagsOf("new-set"),
  isExprStmt: tagsOf("expr-stmt"),
  isVarDecl: tagsOf("var-decl"),
  isFunctionDecl: tagsOf("function-decl"),
  isBlockStmt: tagsOf("block"),
  isIfStmt: tagsOf("if"),
  isWhileStmt: tagsOf("while"),
  isForOfStmt: tagsOf("for-of"),
  isSwitchStmt: tagsOf("switch"),
  isReturnStmt: tagsOf("return"),
  isThrowStmt: tagsOf("throw"),
  isTryStmt: tagsOf("try"),
  isBreakStmt: tagsOf("break"),
  isContinueStmt: tagsOf("continue"),
  isTypeDecl: tagsOf("type-decl", "interface-decl", "import"),
};

const isConsoleCall: Operation<Word> = (args) => {
  const node = formOf(args[0]);
  if (node === undefined || node.tag !== "call") {
    return false;
  }
  const callee = formOf((node as unknown as { callee: Expr }).callee);
  return (
    callee !== undefined &&
    callee.tag === "member" &&
    formOf((callee as unknown as { object: Expr }).object)?.tag === "variable" &&
    (callee as unknown as { object: { name: string } }).object.name === "console" &&
    (callee as unknown as { name: string }).name === "log" &&
    findCell(envOf(args[1]) ?? null, "console") === undefined
  );
};

// ---------------------------------------------------------------------
// The controller: every continuation restores its frame, forwards an
// unresolved transfer to its own continue, then does its work.
// ---------------------------------------------------------------------

const label = (name: string): EvaluatorMachineStatement => ({ tag: "label", name });

/**
 * The evaluator controller over the shared checked syntax: dispatch,
 * operand accumulation, sequencing, and the transfer register for
 * return, throw, break, and loop continue.
 */
export const evaluatorController: ReadonlyArray<EvaluatorMachineStatement> = [
  label("start"),
  assign("expr", op("nextForm")),
  test("isDone", register("expr")),
  branch("done"),
  assign("continue", constant({ tag: "symbol", name: "record-result" })),
  gotoLabel("eval-form"),
  label("record-result"),
  test("isTransfer", register("transfer")),
  branch("done"),
  test("isErrorValue", register("val")),
  branch("record-error"),
  perform("recordResult", register("val")),
  gotoLabel("start"),
  label("record-error"),
  assign("transfer", op("errorTransfer", register("val"))),
  gotoLabel("done"),
  gotoLabel("start"),

  label("eval-form"),
  ...(
    [
      ["isLiteral", "ef-literal"],
      ["isTemplate", "ev-template"],
      ["isVariable", "ef-variable"],
      ["isAssign", "ev-assign"],
      ["isConditionalExpr", "ev-cond-expr"],
      ["isArrow", "ef-arrow"],
      ["isCall", "ev-call"],
      ["isMember", "ev-member"],
      ["isIndex", "ev-index"],
      ["isArray", "ev-array"],
      ["isObject", "ev-object"],
      ["isUnary", "ev-unary"],
      ["isBinary", "ev-binary"],
      ["isLogical", "ev-logical"],
      ["isNewError", "ev-new-error"],
      ["isNewMap", "ev-new-map"],
      ["isNewSet", "ev-new-set"],
      ["isExprStmt", "ef-expr-stmt"],
      ["isVarDecl", "ev-var-decl"],
      ["isFunctionDecl", "ef-function-decl"],
      ["isBlockStmt", "ev-block"],
      ["isIfStmt", "ev-if-stmt"],
      ["isWhileStmt", "ev-while"],
      ["isForOfStmt", "ev-for-of"],
      ["isSwitchStmt", "ev-switch"],
      ["isReturnStmt", "ev-return"],
      ["isThrowStmt", "ev-throw"],
      ["isTryStmt", "ev-try"],
      ["isBreakStmt", "ef-break"],
      ["isContinueStmt", "ef-continue"],
      ["isTypeDecl", "ef-noop"],
    ] as const
  ).flatMap(([predicate, target]): EvaluatorMachineStatement[] => [
    test(predicate, register("expr")),
    branch(target),
  ]),
  gotoLabel("ef-unknown"),

  label("ef-literal"),
  assign("val", op("literalValue", register("expr"))),
  gotoLabel("continue-dispatch"),
  label("ef-variable"),
  assign("val", op("lookupVariableValue", op("variableName", register("expr")), register("env"))),
  test("isErrorValue", register("val")),
  branch("raise-error"),
  gotoLabel("continue-dispatch"),
  label("ef-arrow"),
  assign(
    "val",
    op(
      "makeProcedure",
      op("lambdaParams", register("expr")),
      op("lambdaBody", register("expr")),
      register("env"),
    ),
  ),
  gotoLabel("continue-dispatch"),
  label("ef-expr-stmt"),
  assign("expr", op("exprStmtExpr", register("expr"))),
  gotoLabel("eval-form"),
  label("ef-function-decl"),
  assign("val", op("defineFunction", register("expr"), register("env"))),
  test("isErrorValue", register("val")),
  branch("raise-error"),
  gotoLabel("continue-dispatch"),
  label("ef-break"),
  assign("transfer", op("breakTransfer")),
  gotoLabel("continue-dispatch"),
  label("ef-continue"),
  assign("transfer", op("loopContinueTransfer")),
  gotoLabel("continue-dispatch"),
  label("ef-noop"),
  assign("val", constant(undefined)),
  gotoLabel("continue-dispatch"),
  label("ef-unknown"),
  assign("val", op("unknownFormValue", register("expr"))),
  label("raise-error"),
  assign("transfer", op("throwTransfer", register("val"))),
  gotoLabel("continue-dispatch"),
  label("continue-dispatch"),
  gotoRegister("continue"),

  label("ev-var-decl"),
  save("item"),
  save("continue"),
  assign("item", register("expr")),
  assign("expr", op("varInit", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-var-decl-done" })),
  gotoLabel("eval-form"),
  label("ev-var-decl-done"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-var-decl-exit"),
  test("isErrorValue", register("val")),
  branch("ev-var-decl-raise"),
  assign(
    "val",
    op(
      "declareVariable",
      op("varKindName", register("item")),
      op("variableName", register("item")),
      register("val"),
      register("env"),
    ),
  ),
  test("isErrorValue", register("val")),
  branch("ev-var-decl-raise"),
  gotoLabel("ev-var-decl-exit"),
  label("ev-var-decl-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  label("ev-var-decl-exit"),
  restore("item"),
  gotoLabel("continue-dispatch"),

  label("ev-block"),
  save("env"),
  save("continue"),
  assign("env", op("extendEnvironment", constant([]), constant([]), register("env"))),
  assign("unev", op("blockForms", register("expr"))),
  assign("continue", constant({ tag: "symbol", name: "ev-block-done" })),
  gotoLabel("ev-sequence"),
  label("ev-block-done"),
  restore("continue"),
  restore("env"),
  gotoLabel("continue-dispatch"),

  label("ev-if-stmt"),
  save("item"),
  save("continue"),
  assign("item", register("expr")),
  assign("expr", op("conditionalTest", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-if-stmt-decide" })),
  gotoLabel("eval-form"),
  label("ev-if-stmt-decide"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-if-stmt-exit"),
  test("isErrorValue", register("val")),
  branch("ev-if-stmt-raise"),
  test("truthy", register("val")),
  branch("ev-if-stmt-then"),
  assign("expr", op("ifAlternative", register("item"))),
  gotoLabel("ev-if-stmt-run"),
  label("ev-if-stmt-then"),
  assign("expr", op("ifConsequent", register("item"))),
  label("ev-if-stmt-run"),
  test("isDone", register("expr")),
  branch("ev-if-stmt-none"),
  save("continue"),
  assign("continue", constant({ tag: "symbol", name: "ev-if-stmt-done" })),
  gotoLabel("eval-form"),
  label("ev-if-stmt-none"),
  assign("val", constant(undefined)),
  gotoLabel("ev-if-stmt-exit"),
  label("ev-if-stmt-done"),
  restore("continue"),
  label("ev-if-stmt-exit"),
  restore("item"),
  gotoLabel("continue-dispatch"),
  label("ev-if-stmt-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  gotoLabel("ev-if-stmt-exit"),

  label("ev-cond-expr"),
  test("isTailReturnContinuation", register("continue")),
  branch("ev-cond-tail"),
  save("item"),
  save("continue"),
  assign("item", register("expr")),
  assign("expr", op("conditionalTest", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-cond-decide" })),
  gotoLabel("eval-form"),
  label("ev-cond-decide"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-cond-exit"),
  test("isErrorValue", register("val")),
  branch("ev-cond-raise"),
  test("truthy", register("val")),
  branch("ev-cond-consequent"),
  assign("expr", op("conditionalAlternative", register("item"))),
  gotoLabel("ev-cond-run"),
  label("ev-cond-consequent"),
  assign("expr", op("conditionalConsequent", register("item"))),
  label("ev-cond-run"),
  save("continue"),
  assign("continue", constant({ tag: "symbol", name: "ev-cond-done" })),
  gotoLabel("eval-form"),
  label("ev-cond-done"),
  restore("continue"),
  label("ev-cond-exit"),
  restore("item"),
  gotoLabel("continue-dispatch"),
  label("ev-cond-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  gotoLabel("ev-cond-exit"),
  label("ev-cond-tail"),
  assign("item", register("expr")),
  assign("expr", op("conditionalTest", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-cond-tail-decide" })),
  gotoLabel("eval-form"),
  label("ev-cond-tail-decide"),
  test("isTransfer", register("transfer")),
  branch("ev-cond-tail-exit"),
  test("isErrorValue", register("val")),
  branch("ev-cond-tail-raise"),
  test("truthy", register("val")),
  branch("ev-cond-tail-consequent"),
  assign("expr", op("conditionalAlternative", register("item"))),
  gotoLabel("ev-cond-tail-run"),
  label("ev-cond-tail-consequent"),
  assign("expr", op("conditionalConsequent", register("item"))),
  label("ev-cond-tail-run"),
  assign("continue", constant({ tag: "symbol", name: "tail-return" })),
  gotoLabel("eval-form"),
  label("ev-cond-tail-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  label("ev-cond-tail-exit"),
  assign("continue", constant({ tag: "symbol", name: "apply-body-done" })),
  gotoLabel("apply-body-done"),

  label("ev-assign"),
  test("isVariable", op("assignTarget", register("expr"))),
  branch("ev-assign-var"),
  test("isMember", op("assignTarget", register("expr"))),
  branch("ev-assign-member"),
  gotoLabel("ev-assign-index"),
  label("ev-assign-var"),
  save("item"),
  save("continue"),
  assign("item", register("expr")),
  assign("expr", op("assignValue", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-assign-var-done" })),
  gotoLabel("eval-form"),
  label("ev-assign-var-done"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-assign-var-exit"),
  test("isErrorValue", register("val")),
  branch("ev-assign-var-raise"),
  assign(
    "val",
    op(
      "setVariableValue",
      op("assignTargetName", register("item")),
      register("val"),
      register("env"),
    ),
  ),
  test("isErrorValue", register("val")),
  branch("ev-assign-var-raise"),
  gotoLabel("ev-assign-var-exit"),
  label("ev-assign-var-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  label("ev-assign-var-exit"),
  restore("item"),
  gotoLabel("continue-dispatch"),

  label("ev-assign-member"),
  save("item"),
  save("continue"),
  assign("item", register("expr")),
  assign("expr", op("memberObject", op("assignTarget", register("item")))),
  assign("continue", constant({ tag: "symbol", name: "ev-assign-member-obj" })),
  gotoLabel("eval-form"),
  label("ev-assign-member-obj"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-assign-member-exit"),
  test("isErrorValue", register("val")),
  branch("ev-assign-member-raise"),
  save("proc"),
  save("continue"),
  assign("proc", register("val")),
  assign("expr", op("assignValue", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-assign-member-val" })),
  gotoLabel("eval-form"),
  label("ev-assign-member-val"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-assign-member-forward"),
  test("isErrorValue", register("val")),
  branch("ev-assign-member-inner-raise"),
  assign(
    "val",
    op(
      "memberSet",
      register("proc"),
      op("memberName", op("assignTarget", register("item"))),
      register("val"),
    ),
  ),
  test("isErrorValue", register("val")),
  branch("ev-assign-member-inner-raise"),
  restore("proc"),
  gotoLabel("ev-assign-member-exit"),
  label("ev-assign-member-inner-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  label("ev-assign-member-forward"),
  restore("proc"),
  gotoLabel("ev-assign-member-exit"),
  label("ev-assign-member-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  label("ev-assign-member-exit"),
  restore("item"),
  gotoLabel("continue-dispatch"),

  label("ev-assign-index"),
  save("item"),
  save("continue"),
  assign("item", register("expr")),
  assign("expr", op("indexObject", op("assignTarget", register("item")))),
  assign("continue", constant({ tag: "symbol", name: "ev-assign-index-obj" })),
  gotoLabel("eval-form"),
  label("ev-assign-index-obj"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-assign-index-exit"),
  test("isErrorValue", register("val")),
  branch("ev-assign-index-raise"),
  save("proc"),
  save("argl"),
  save("continue"),
  assign("proc", register("val")),
  assign("expr", op("indexIndex", op("assignTarget", register("item")))),
  assign("continue", constant({ tag: "symbol", name: "ev-assign-index-idx" })),
  gotoLabel("eval-form"),
  label("ev-assign-index-idx"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-assign-index-forward"),
  test("isErrorValue", register("val")),
  branch("ev-assign-index-inner-raise"),
  save("continue"),
  assign("argl", register("val")),
  assign("expr", op("assignValue", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-assign-index-val" })),
  gotoLabel("eval-form"),
  label("ev-assign-index-val"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-assign-index-forward"),
  test("isErrorValue", register("val")),
  branch("ev-assign-index-inner-raise"),
  assign("val", op("indexSet", register("proc"), register("argl"), register("val"))),
  test("isErrorValue", register("val")),
  branch("ev-assign-index-inner-raise"),
  restore("argl"),
  restore("proc"),
  gotoLabel("ev-assign-index-exit"),
  label("ev-assign-index-inner-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  label("ev-assign-index-forward"),
  restore("argl"),
  restore("proc"),
  gotoLabel("ev-assign-index-exit"),
  label("ev-assign-index-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  label("ev-assign-index-exit"),
  restore("item"),
  gotoLabel("continue-dispatch"),

  label("ev-member"),
  save("item"),
  save("continue"),
  assign("item", register("expr")),
  assign("expr", op("memberObject", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-member-done" })),
  gotoLabel("eval-form"),
  label("ev-member-done"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-member-exit"),
  test("isErrorValue", register("val")),
  branch("ev-member-raise"),
  assign("val", op("memberGet", register("val"), op("memberName", register("item")))),
  test("isErrorValue", register("val")),
  branch("ev-member-raise"),
  gotoLabel("ev-member-exit"),
  label("ev-member-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  label("ev-member-exit"),
  restore("item"),
  gotoLabel("continue-dispatch"),

  label("ev-index"),
  save("item"),
  save("continue"),
  assign("item", register("expr")),
  assign("expr", op("indexObject", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-index-obj" })),
  gotoLabel("eval-form"),
  label("ev-index-obj"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-index-exit"),
  test("isErrorValue", register("val")),
  branch("ev-index-raise"),
  save("proc"),
  save("continue"),
  assign("proc", register("val")),
  assign("expr", op("indexIndex", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-index-done" })),
  gotoLabel("eval-form"),
  label("ev-index-done"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-index-forward"),
  test("isErrorValue", register("val")),
  branch("ev-index-inner-raise"),
  assign("val", op("indexGet", register("proc"), register("val"))),
  test("isErrorValue", register("val")),
  branch("ev-index-inner-raise"),
  restore("proc"),
  gotoLabel("ev-index-exit"),
  label("ev-index-inner-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  label("ev-index-forward"),
  restore("proc"),
  gotoLabel("ev-index-exit"),
  label("ev-index-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  label("ev-index-exit"),
  restore("item"),
  gotoLabel("continue-dispatch"),

  label("ev-unary"),
  save("item"),
  save("continue"),
  assign("item", register("expr")),
  assign("expr", op("unaryOperand", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-unary-done" })),
  gotoLabel("eval-form"),
  label("ev-unary-done"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-unary-exit"),
  test("isErrorValue", register("val")),
  branch("ev-unary-raise"),
  assign("val", op("unaryValue", op("unaryOperator", register("item")), register("val"))),
  test("isErrorValue", register("val")),
  branch("ev-unary-raise"),
  gotoLabel("ev-unary-exit"),
  label("ev-unary-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  label("ev-unary-exit"),
  restore("item"),
  gotoLabel("continue-dispatch"),

  label("ev-binary"),
  save("item"),
  save("continue"),
  assign("item", register("expr")),
  assign("expr", op("binaryLeft", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-binary-left" })),
  gotoLabel("eval-form"),
  label("ev-binary-left"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-binary-exit"),
  test("isErrorValue", register("val")),
  branch("ev-binary-raise"),
  save("proc"),
  save("continue"),
  assign("proc", register("val")),
  assign("expr", op("binaryRight", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-binary-right" })),
  gotoLabel("eval-form"),
  label("ev-binary-right"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-binary-forward"),
  test("isErrorValue", register("val")),
  branch("ev-binary-inner-raise"),
  assign(
    "val",
    op("binaryValue", op("binaryOperator", register("item")), register("proc"), register("val")),
  ),
  test("isErrorValue", register("val")),
  branch("ev-binary-inner-raise"),
  restore("proc"),
  gotoLabel("ev-binary-exit"),
  label("ev-binary-inner-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  label("ev-binary-forward"),
  restore("proc"),
  gotoLabel("ev-binary-exit"),
  label("ev-binary-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  label("ev-binary-exit"),
  restore("item"),
  gotoLabel("continue-dispatch"),

  label("ev-logical"),
  save("item"),
  save("continue"),
  assign("item", register("expr")),
  assign("expr", op("binaryLeft", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-logical-left" })),
  gotoLabel("eval-form"),
  label("ev-logical-left"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-logical-exit"),
  test("isErrorValue", register("val")),
  branch("ev-logical-raise"),
  test("logicalSkipRight", op("binaryOperator", register("item")), register("val")),
  branch("ev-logical-exit"),
  save("continue"),
  assign("expr", op("binaryRight", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-logical-done" })),
  gotoLabel("eval-form"),
  label("ev-logical-done"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-logical-exit"),
  test("isErrorValue", register("val")),
  branch("ev-logical-raise"),
  gotoLabel("ev-logical-exit"),
  label("ev-logical-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  label("ev-logical-exit"),
  restore("item"),
  gotoLabel("continue-dispatch"),

  label("ev-array"),
  save("item"),
  save("argl"),
  save("continue"),
  assign("item", register("expr")),
  assign("argl", op("emptyArgList")),
  assign("unev", op("arrayElements", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-array-done" })),
  gotoLabel("ev-operand-loop"),
  label("ev-array-done"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-collection-exit"),
  assign("val", op("arrayValue", register("argl"))),
  gotoLabel("ev-collection-exit"),

  label("ev-object"),
  save("item"),
  save("argl"),
  save("continue"),
  assign("item", register("expr")),
  assign("argl", op("emptyArgList")),
  assign("unev", op("objectValues", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-object-done" })),
  gotoLabel("ev-operand-loop"),
  label("ev-object-done"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-collection-exit"),
  assign("val", op("objectValue", op("objectKeys", register("item")), register("argl"))),
  gotoLabel("ev-collection-exit"),

  label("ev-template"),
  save("item"),
  save("argl"),
  save("continue"),
  assign("item", register("expr")),
  assign("argl", op("emptyArgList")),
  assign("unev", op("templateExprs", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-template-done" })),
  gotoLabel("ev-operand-loop"),
  label("ev-template-done"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-collection-exit"),
  assign("val", op("templateValue", op("templateChunks", register("item")), register("argl"))),
  gotoLabel("ev-collection-exit"),

  label("ev-new-error"),
  save("item"),
  save("argl"),
  save("continue"),
  assign("item", register("expr")),
  assign("argl", op("emptyArgList")),
  assign("unev", op("argExprs", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-new-error-done" })),
  gotoLabel("ev-operand-loop"),
  label("ev-new-error-done"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-collection-exit"),
  assign("val", op("newErrorValue", register("argl"))),
  gotoLabel("ev-collection-exit"),

  label("ev-new-map"),
  save("item"),
  save("argl"),
  save("continue"),
  assign("item", register("expr")),
  assign("argl", op("emptyArgList")),
  assign("unev", op("argExprs", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-new-map-done" })),
  gotoLabel("ev-operand-loop"),
  label("ev-new-map-done"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-collection-exit"),
  assign("val", op("newMapValue", register("argl"))),
  gotoLabel("ev-collection-exit"),

  label("ev-new-set"),
  save("item"),
  save("argl"),
  save("continue"),
  assign("item", register("expr")),
  assign("argl", op("emptyArgList")),
  assign("unev", op("argExprs", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-new-set-done" })),
  gotoLabel("ev-operand-loop"),
  label("ev-new-set-done"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-collection-exit"),
  assign("val", op("newSetValue", register("argl"))),
  label("ev-collection-exit"),
  restore("argl"),
  restore("item"),
  gotoLabel("continue-dispatch"),

  label("ev-call"),
  test("isConsoleCall", register("expr"), register("env")),
  branch("ev-output"),
  test("isTailReturnContinuation", register("continue")),
  branch("ev-call-tail"),
  save("item"),
  save("proc"),
  save("argl"),
  save("continue"),
  assign("item", register("expr")),
  assign("expr", op("calleeOf", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-call-proc" })),
  gotoLabel("eval-form"),
  label("ev-call-tail"),
  assign("item", register("expr")),
  assign("expr", op("calleeOf", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-call-tail-proc" })),
  gotoLabel("eval-form"),
  label("ev-call-proc"),
  test("isTransfer", register("transfer")),
  branch("call-forward"),
  test("isErrorValue", register("val")),
  branch("call-raise"),
  assign("proc", register("val")),
  assign("argl", op("emptyArgList")),
  assign("unev", op("argExprs", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-call-args-done" })),
  gotoLabel("ev-operand-loop"),
  label("ev-call-args-done"),
  test("isTransfer", register("transfer")),
  branch("call-forward"),
  gotoLabel("apply-dispatch"),
  label("ev-call-tail-proc"),
  test("isTransfer", register("transfer")),
  branch("ev-call-tail-forward"),
  test("isErrorValue", register("val")),
  branch("ev-call-tail-raise"),
  assign("proc", register("val")),
  assign("argl", op("emptyArgList")),
  assign("unev", op("argExprs", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-call-tail-args-done" })),
  gotoLabel("ev-operand-loop"),
  label("ev-call-tail-args-done"),
  test("isTransfer", register("transfer")),
  branch("ev-call-tail-forward"),
  assign("tailCall", constant(true)),
  gotoLabel("apply-dispatch"),
  label("ev-call-tail-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  label("ev-call-tail-forward"),
  assign("tailCall", constant(true)),
  gotoLabel("call-forward"),
  label("apply-dispatch"),
  test("isPrimitiveProcedure", register("proc")),
  branch("apply-primitive"),
  test("isClosure", register("proc")),
  branch("apply-compound"),
  assign("val", op("unknownProcedureValue", register("proc"))),
  gotoLabel("call-raise"),
  label("apply-primitive"),
  assign("val", op("applyPrimitiveProcedure", register("proc"), register("argl"))),
  test("isErrorValue", register("val")),
  branch("call-raise"),
  test("isTailCall", register("tailCall")),
  branch("apply-tail-primitive"),
  gotoLabel("call-end"),
  label("apply-tail-primitive"),
  assign("tailCall", constant(false)),
  assign("continue", constant({ tag: "symbol", name: "apply-body-done" })),
  gotoLabel("apply-body-done"),
  label("apply-compound"),
  test("isTailCall", register("tailCall")),
  branch("apply-tail-compound"),
  save("env"),
  assign(
    "env",
    op(
      "extendEnvironment",
      op("procedureParams", register("proc")),
      register("argl"),
      op("procedureEnv", register("proc")),
      op("procedureRest", register("proc")),
    ),
  ),
  assign("unev", op("procedureBody", register("proc"))),
  assign("continue", constant({ tag: "symbol", name: "apply-body-done" })),
  gotoLabel("ev-sequence"),
  label("apply-tail-compound"),
  assign("tailCall", constant(false)),
  assign(
    "env",
    op(
      "extendEnvironment",
      op("procedureParams", register("proc")),
      register("argl"),
      op("procedureEnv", register("proc")),
      op("procedureRest", register("proc")),
    ),
  ),
  assign("unev", op("procedureBody", register("proc"))),
  assign("continue", constant({ tag: "symbol", name: "apply-body-done" })),
  gotoLabel("ev-sequence"),
  label("apply-body-done"),
  restore("env"),
  restore("continue"),
  test("isReturnTransfer", register("transfer")),
  branch("apply-return-clear"),
  gotoLabel("apply-body-exit"),
  label("apply-return-clear"),
  assign("transfer", op("nullTransfer")),
  label("apply-body-exit"),
  restore("argl"),
  restore("proc"),
  restore("item"),
  gotoLabel("continue-dispatch"),
  label("call-end"),
  restore("continue"),
  restore("argl"),
  restore("proc"),
  restore("item"),
  gotoLabel("continue-dispatch"),
  label("call-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  label("call-forward"),
  test("isTailCall", register("tailCall")),
  branch("call-tail-forward"),
  restore("continue"),
  restore("argl"),
  restore("proc"),
  restore("item"),
  gotoLabel("continue-dispatch"),
  label("call-tail-forward"),
  assign("tailCall", constant(false)),
  assign("continue", constant({ tag: "symbol", name: "apply-body-done" })),
  gotoLabel("apply-body-done"),

  label("ev-output"),
  save("item"),
  save("argl"),
  save("continue"),
  assign("item", register("expr")),
  assign("argl", op("emptyArgList")),
  assign("unev", op("argExprs", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-output-done" })),
  gotoLabel("ev-operand-loop"),
  label("ev-output-done"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-output-exit"),
  perform("recordOutput", register("argl")),
  assign("val", constant(undefined)),
  label("ev-output-exit"),
  restore("argl"),
  restore("item"),
  gotoLabel("continue-dispatch"),

  label("ev-operand-loop"),
  test("noOperands", register("unev")),
  branch("continue-dispatch"),
  save("unev"),
  save("continue"),
  assign("expr", op("firstOperand", register("unev"))),
  assign("continue", constant({ tag: "symbol", name: "ev-operand-next" })),
  gotoLabel("eval-form"),
  label("ev-operand-next"),
  restore("continue"),
  restore("unev"),
  test("isTransfer", register("transfer")),
  branch("continue-dispatch"),
  test("isErrorValue", register("val")),
  branch("raise-error"),
  assign("argl", op("adjoinArg", register("argl"), register("unev"), register("val"))),
  assign("unev", op("restOperands", register("unev"))),
  gotoLabel("ev-operand-loop"),

  label("ev-return"),
  test("isApplyBodyContinuation", register("continue")),
  branch("ev-return-tail"),
  save("item"),
  save("continue"),
  assign("item", register("expr")),
  assign("expr", op("returnArgument", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-return-done" })),
  test("isDone", register("expr")),
  branch("ev-return-bare"),
  gotoLabel("eval-form"),
  label("ev-return-bare"),
  assign("val", constant(undefined)),
  label("ev-return-done"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-return-exit"),
  test("isErrorValue", register("val")),
  branch("ev-return-raise"),
  assign("transfer", op("returnTransfer")),
  gotoLabel("ev-return-exit"),
  label("ev-return-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  label("ev-return-exit"),
  restore("item"),
  gotoLabel("continue-dispatch"),
  label("ev-return-tail"),
  assign("expr", op("returnArgument", register("expr"))),
  test("isDone", register("expr")),
  branch("ev-return-tail-bare"),
  assign("continue", constant({ tag: "symbol", name: "tail-return" })),
  gotoLabel("eval-form"),
  label("ev-return-tail-bare"),
  assign("val", constant(undefined)),
  gotoLabel("apply-body-done"),
  label("tail-return"),
  assign("continue", constant({ tag: "symbol", name: "apply-body-done" })),
  gotoLabel("apply-body-done"),

  label("ev-throw"),
  save("item"),
  save("continue"),
  assign("item", register("expr")),
  assign("expr", op("throwArgument", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-throw-done" })),
  gotoLabel("eval-form"),
  label("ev-throw-done"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("ev-throw-exit"),
  assign("transfer", op("throwTransfer", register("val"))),
  label("ev-throw-exit"),
  restore("item"),
  gotoLabel("continue-dispatch"),

  label("ev-while"),
  save("item"),
  save("continue"),
  assign("item", register("expr")),
  label("while-test"),
  save("continue"),
  assign("expr", op("conditionalTest", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "while-decide" })),
  gotoLabel("eval-form"),
  label("while-decide"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("while-exit"),
  test("isErrorValue", register("val")),
  branch("while-raise"),
  test("truthy", register("val")),
  branch("while-body"),
  gotoLabel("while-exit"),
  label("while-body"),
  save("continue"),
  assign("expr", op("whileBody", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "while-continue" })),
  gotoLabel("eval-form"),
  label("while-continue"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("while-transfer"),
  gotoLabel("while-test"),
  label("while-transfer"),
  test("isBreakTransfer", register("transfer")),
  branch("while-break"),
  test("isLoopContinueTransfer", register("transfer")),
  branch("while-next"),
  gotoLabel("while-exit"),
  label("while-break"),
  assign("transfer", op("nullTransfer")),
  gotoLabel("while-exit"),
  label("while-next"),
  assign("transfer", op("nullTransfer")),
  gotoLabel("while-test"),
  label("while-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  label("while-exit"),
  restore("continue"),
  restore("item"),
  gotoLabel("continue-dispatch"),

  label("ev-for-of"),
  save("item"),
  save("continue"),
  assign("item", register("expr")),
  save("continue"),
  assign("expr", op("forOfIterable", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "ev-for-of-iter" })),
  gotoLabel("eval-form"),
  label("ev-for-of-iter"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("for-of-exit"),
  test("isErrorValue", register("val")),
  branch("for-of-raise"),
  assign("proc", op("makeIteration", register("val"))),
  gotoLabel("for-of-next"),
  label("for-of-next"),
  test("iterationDone", register("proc")),
  branch("for-of-exit"),
  save("env"),
  save("continue"),
  save("proc"),
  assign(
    "env",
    op(
      "extendEnvironment",
      op("forOfName", register("item")),
      op("iterationItem", register("proc")),
      register("env"),
    ),
  ),
  assign("expr", op("forOfBody", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "for-of-continue" })),
  gotoLabel("eval-form"),
  label("for-of-continue"),
  restore("proc"),
  restore("continue"),
  restore("env"),
  test("isTransfer", register("transfer")),
  branch("for-of-transfer"),
  assign("proc", op("iterationNext", register("proc"))),
  gotoLabel("for-of-next"),
  label("for-of-transfer"),
  test("isBreakTransfer", register("transfer")),
  branch("for-of-break"),
  test("isLoopContinueTransfer", register("transfer")),
  branch("for-of-advance"),
  gotoLabel("for-of-exit"),
  label("for-of-break"),
  assign("transfer", op("nullTransfer")),
  gotoLabel("for-of-exit"),
  label("for-of-advance"),
  assign("transfer", op("nullTransfer")),
  assign("proc", op("iterationNext", register("proc"))),
  gotoLabel("for-of-next"),
  label("for-of-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  label("for-of-exit"),
  restore("continue"),
  restore("item"),
  gotoLabel("continue-dispatch"),

  label("ev-switch"),
  save("item"),
  save("proc"),
  save("argl"),
  save("continue"),
  assign("item", register("expr")),
  save("continue"),
  assign("expr", op("switchDiscriminant", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "switch-scan" })),
  gotoLabel("eval-form"),
  label("switch-scan"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("switch-exit"),
  test("isErrorValue", register("val")),
  branch("switch-raise"),
  assign("proc", register("val")),
  assign("unev", op("switchCases", register("item"))),
  assign("argl", constant(0)),
  gotoLabel("switch-next"),
  label("switch-next"),
  test("noCases", register("unev")),
  branch("switch-default"),
  save("continue"),
  assign("expr", op("caseTest", register("unev"))),
  assign("continue", constant({ tag: "symbol", name: "switch-case-decide" })),
  gotoLabel("eval-form"),
  label("switch-case-decide"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("switch-exit"),
  test("isErrorValue", register("val")),
  branch("switch-raise"),
  assign("val", op("binaryValue", constant("==="), register("proc"), register("val"))),
  test("truthy", register("val")),
  branch("switch-run"),
  assign("unev", op("restCases", register("unev"))),
  assign("argl", op("binaryValue", constant("+"), register("argl"), constant(1))),
  gotoLabel("switch-next"),
  label("switch-default"),
  assign("unev", op("switchDefaultBody", register("item"))),
  gotoLabel("switch-scope"),
  label("switch-run"),
  assign("unev", op("caseBodiesFrom", register("item"), register("argl"))),
  label("switch-scope"),
  save("env"),
  assign("env", op("extendEnvironment", constant([]), constant([]), register("env"))),
  perform("predeclareForms", op("caseBodiesFrom", register("item"), constant(0)), register("env")),
  save("continue"),
  assign("continue", constant({ tag: "symbol", name: "switch-done" })),
  gotoLabel("ev-sequence"),
  label("switch-done"),
  restore("continue"),
  restore("env"),
  test("isBreakTransfer", register("transfer")),
  branch("switch-break"),
  gotoLabel("switch-exit"),
  label("switch-break"),
  assign("transfer", op("nullTransfer")),
  gotoLabel("switch-exit"),
  label("switch-raise"),
  assign("transfer", op("throwTransfer", register("val"))),
  label("switch-exit"),
  restore("continue"),
  restore("argl"),
  restore("proc"),
  restore("item"),
  gotoLabel("continue-dispatch"),

  label("ev-try"),
  save("item"),
  save("continue"),
  assign("item", register("expr")),
  save("continue"),
  assign("expr", op("tryBlock", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "try-body-done" })),
  gotoLabel("eval-form"),
  label("try-body-done"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("try-pending"),
  gotoLabel("try-finally"),
  label("try-pending"),
  test("isThrowTransfer", register("transfer")),
  branch("try-handler-check"),
  gotoLabel("try-finally"),
  label("try-handler-check"),
  test("hasHandler", register("item")),
  branch("try-catch"),
  gotoLabel("try-finally"),
  label("try-catch"),
  assign("val", op("transferValue", register("transfer"))),
  assign("transfer", op("nullTransfer")),
  save("env"),
  assign(
    "env",
    op(
      "extendEnvironment",
      op("tryHandlerParam", register("item")),
      register("val"),
      register("env"),
    ),
  ),
  save("continue"),
  assign("expr", op("tryHandlerBody", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "try-handler-done" })),
  gotoLabel("eval-form"),
  label("try-handler-done"),
  restore("continue"),
  restore("env"),
  gotoLabel("try-finally"),
  label("try-finally"),
  test("hasFinalizer", register("item")),
  branch("try-finally-run"),
  gotoLabel("try-exit"),
  label("try-finally-run"),
  save("transfer"),
  assign("transfer", op("nullTransfer")),
  save("val"),
  save("continue"),
  assign("expr", op("tryFinalizerBody", register("item"))),
  assign("continue", constant({ tag: "symbol", name: "try-finally-done" })),
  gotoLabel("eval-form"),
  label("try-finally-done"),
  restore("continue"),
  test("isTransfer", register("transfer")),
  branch("try-finally-abrupt"),
  restore("val"),
  restore("transfer"),
  gotoLabel("try-exit"),
  label("try-finally-abrupt"),
  assign("unev", register("transfer")),
  assign("argl", register("val")),
  restore("val"),
  restore("transfer"),
  assign("transfer", register("unev")),
  assign("val", register("argl")),
  gotoLabel("try-exit"),
  label("try-exit"),
  restore("continue"),
  restore("item"),
  gotoLabel("continue-dispatch"),

  label("ev-sequence"),
  perform("predeclareForms", register("unev"), register("env")),
  test("noOperands", register("unev")),
  branch("seq-empty"),
  assign("expr", op("firstOperand", register("unev"))),
  assign("unev", op("restOperands", register("unev"))),
  test("noOperands", register("unev")),
  branch("seq-last"),
  save("unev"),
  save("continue"),
  assign("continue", constant({ tag: "symbol", name: "seq-next" })),
  gotoLabel("eval-form"),
  label("seq-next"),
  restore("continue"),
  restore("unev"),
  test("isTransfer", register("transfer")),
  branch("continue-dispatch"),
  test("isErrorValue", register("val")),
  branch("raise-error"),
  gotoLabel("ev-sequence"),
  label("seq-last"),
  gotoLabel("eval-form"),
  label("seq-empty"),
  assign("val", constant(undefined)),
  gotoLabel("continue-dispatch"),
  label("done"),
];

// ---------------------------------------------------------------------
// Machine construction and the run entry points.
// ---------------------------------------------------------------------

/**
 * The run's step bound: a guard against non-terminating guest programs. It
 * sits above legitimate deep recursion (the controller spends roughly 450
 * steps per non-tail guest call, so 5000-deep recursion is about 2.3 million
 * steps) and below the point where the machine's per-step trace exhausts the
 * heap on a diverging program.
 */
const EVALUATOR_STEP_LIMIT = 4_000_000;

/** Builds the explicit-control evaluator over one admitted unit. */
export const makeEvaluator = (
  source: string,
  customOperations: Readonly<Record<string, Operation<Word>>> = {},
  controller: ReadonlyArray<EvaluatorMachineStatement> = evaluatorController,
  modules: LinkedModules = {},
): Evaluator => {
  const session = new Session("core", modules);
  const state: EvaluatorState = { session, forms: [], next: 0, values: [] };
  const machine = makeMachine<Word>({
    registers: [
      "expr",
      "env",
      "val",
      "proc",
      "argl",
      "unev",
      "continue",
      "transfer",
      "item",
      "tailCall",
    ],
    operations: { ...operationsFor(state), ...predicates, isConsoleCall, ...customOperations },
    controller,
  });
  return {
    machine,
    run(): RunResult {
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
      state.forms = [...admission.program];
      state.next = 0;
      state.values = [];
      const environment = session.globalEnv();
      for (const form of admission.program) {
        // Imports link before the first form runs, like module instantiation.
        const error = form.tag === "import" ? session.linkImport(form, environment) : null;
        if (error !== null) {
          return { outcome: fail(error), transcript: [] };
        }
      }
      predeclare(admission.program, environment);
      machine.writeRegister("env", environment);
      machine.writeRegister("transfer", undefined);
      const run = machine.run(EVALUATOR_STEP_LIMIT);
      const transfer = machine.readRegister("transfer");
      const pending =
        typeof transfer === "object" && transfer !== null && "kind" in transfer
          ? (transfer as Transfer)
          : undefined;
      if (run.error !== null) {
        const detail = run.error.tag === "unknown-operation" ? run.error.name : run.error.tag;
        return {
          outcome: fail({ tag: "unknown-syntax", construct: `machine-error/${detail}` }),
          transcript: session.transcript,
        };
      }
      if (pending?.kind === "error") {
        return { outcome: fail(pending.error), transcript: session.transcript };
      }
      if (pending?.kind === "throw") {
        return {
          outcome: fail({ tag: "guest-throw", value: pending.value }),
          transcript: session.transcript,
        };
      }
      return { outcome: ok(state.values[state.values.length - 1]), transcript: session.transcript };
    },
  };
};

/** Runs one admitted unit on the explicit-control machine. */
export const runEvaluator = (
  source: string,
  customOperations: Readonly<Record<string, Operation<Word>>> = {},
  modules: LinkedModules = {},
): RunResult => makeEvaluator(source, customOperations, evaluatorController, modules).run();

/** Renders the machine trace of one evaluator run (the 5.4 monitoring exercises). */
export const renderTrace = (machine: Machine<Word>): ReadonlyArray<string> =>
  machine.result().trace;

/** The monitored run: transcript plus stack statistics. */
export interface MeasuredRun extends RunResult {
  readonly stackStats: { readonly pushes: number; readonly maxDepth: number };
  readonly instructionCount: number;
}

/** Runs one unit and reports the machine's stack statistics (5.4.4). */
export const runMonitoredEvaluator = (
  source: string,
  controller: ReadonlyArray<EvaluatorMachineStatement> = evaluatorController,
): MeasuredRun => {
  const evaluator = makeEvaluator(source, {}, controller);
  const result = evaluator.run();
  const machine = evaluator.machine.result();
  return {
    ...result,
    stackStats: machine.stackStats,
    instructionCount: machine.instructionCount,
  };
};

/** Replaces one labeled segment of a controller copy (5.4 monitoring exercises). */
export const replaceSegment = (
  lines: ReadonlyArray<EvaluatorMachineStatement>,
  fromLabel: string,
  toLabelExclusive: string,
  replacement: ReadonlyArray<EvaluatorMachineStatement>,
): EvaluatorMachineStatement[] => {
  const start = lines.findIndex((line) => line.tag === "label" && line.name === fromLabel);
  const end = lines.findIndex((line) => line.tag === "label" && line.name === toLabelExclusive);
  if (start < 0 || end < 0 || end < start) {
    return [...lines];
  }
  return [...lines.slice(0, start), ...replacement, ...lines.slice(end)];
};

/** Inserts controller lines before the first matching instruction. */
export const insertBeforeInstruction = (
  lines: ReadonlyArray<EvaluatorMachineStatement>,
  matches: (line: EvaluatorMachineStatement) => boolean,
  description: string,
  insertions: ReadonlyArray<EvaluatorMachineStatement>,
): EvaluatorMachineStatement[] => {
  void description;
  const at = lines.findIndex(matches);
  return at < 0 ? [...lines] : [...lines.slice(0, at), ...insertions, ...lines.slice(at)];
};

/** Appends controller lines to a copy. */
export const appendLines = (
  lines: ReadonlyArray<EvaluatorMachineStatement>,
  additions: ReadonlyArray<EvaluatorMachineStatement>,
): EvaluatorMachineStatement[] => [...lines, ...additions];

/** The evaluator's value renderer for transcripts. */
export const formatValue = format;

export type { GenericMachineStatement as MachineStatement, Operation };
// Re-exported construction helpers used by controller-variant exercises.
export {
  assign,
  branch,
  constant,
  gotoLabel,
  gotoRegister,
  Machine,
  makeMachine,
  op,
  perform,
  register,
  restore,
  save,
  test,
};
