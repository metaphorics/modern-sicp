// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.1

import { format } from "./read.ts";
import {
  MATH_NAMES,
  makeBuiltins,
  NUMBER_NAMES,
  namespaceValue,
  type OpTable,
} from "./runtime/builtins.ts";
import { child, type Env, findCell, makeCell } from "./runtime/env.ts";
import {
  type Completion,
  fail,
  type GuestError,
  normal,
  type Outcome,
  ok,
} from "./runtime/errors.ts";
import {
  type ArrayValue,
  type Closure,
  isArrayValue,
  isClosure,
  isErrorValue,
  isMapValue,
  isPrimitive,
  isRecordValue,
  isSetValue,
  isThunkValue,
  makeArray,
  makeClosure,
  makeErrorValue,
  makeMap,
  makeRecord,
  makeSet,
  ThunkValue,
  type Value,
} from "./runtime/value.ts";
/**
 * The direct evaluator and the analyzer (host-subsets grammar sections 3, 5,
 * and 6): one case analysis over the shared checked AST, with lexical cells,
 * TDZ reads, shared captured writes, left-to-right evaluation order, and the
 * declared `Outcome` error channel — no host exceptions cross the evaluator
 * boundary and no host evaluator runs guest code. `analyze` separates
 * syntactic analysis from execution (4.1.7): it traverses the syntax once and
 * returns an execution procedure that runs against many environments. The
 * transcript is data the session collects; only boundary drivers print it.
 */
import type { Arg, Decl, Expr, Param, Program, Stmt } from "./syntax/ast.ts";
import { admitSource } from "./syntax/check.ts";
import type { ExperimentMode } from "./syntax/parse.ts";

/** One engine run: the outcome plus the ordered transcript it produced. */
export interface RunResult {
  readonly outcome: Outcome;
  readonly transcript: ReadonlyArray<string>;
}

/** An execution procedure: analysis done, only the environment remains. */
export type ExecutionProcedure = (env: Env) => Outcome;
export type { Outcome } from "./runtime/errors.ts";

/** The result of extending an environment with new bindings. */
export type EnvOutcome =
  | { readonly tag: "ok"; readonly env: Env }
  | { readonly tag: "error"; readonly error: GuestError };

/** Evaluated arguments or the error that stopped them. */
type ArgsOutcome =
  | { readonly tag: "args"; readonly values: ReadonlyArray<Value> }
  | { readonly tag: "error"; readonly error: GuestError };

const bad = (operator: string, detail: string): Outcome =>
  fail({ tag: "bad-operand", operator, detail });

/** Array methods that must call guest procedures; they live in the evaluator. */
const CALLBACK_METHODS: Readonly<Record<string, true>> = {
  map: true,
  flatMap: true,
  filter: true,
  find: true,
  some: true,
  every: true,
  reduce: true,
  reduceRight: true,
  forEach: true,
};

/** One evaluator session: modes, transcript, and experiment counters. */
export class Session {
  readonly mode: ExperimentMode;
  readonly transcript: string[] = [];
  /** How many thunk computations ran (the 4.2 counting exercises). */
  evaluations = 0;
  readonly #builtins: OpTable = makeBuiltins();
  readonly #globals = new Map<string, Value>();

  constructor(mode: ExperimentMode) {
    this.mode = mode;
    this.#globals.set("Math", namespaceValue(this.#builtins, "math", MATH_NAMES));
    this.#globals.set("Number", namespaceValue(this.#builtins, "number", NUMBER_NAMES));
  }

  /** The session's global environment: builtins plus namespace values. */
  globalEnv(): Env {
    const env = child(null);
    for (const [name, value] of this.#globals) {
      env.bindings.set(name, makeCell(value, true));
    }
    return env;
  }

  // ------------------------------------------------------------------
  // Expressions
  // ------------------------------------------------------------------

  evaluate(expr: Expr, env: Env): Outcome {
    switch (expr.tag) {
      case "number":
      case "string":
      case "boolean":
        return ok(expr.value);
      case "null":
        return ok(null);
      case "undefined":
        return ok(undefined);
      case "template":
        return this.evalTemplate(expr.chunks, expr.exprs, env);
      case "variable":
        return this.lookupVariableValue(expr.name, env);
      case "array":
        return this.evalArray(expr.elements, env);
      case "object":
        return this.evalObject(expr.fields, env);
      case "unary":
        return this.evalUnary(expr.op, expr.operand, env);
      case "binary":
        return this.evalBinary(expr.op, expr.left, expr.right, env);
      case "logical":
        return this.evalLogical(expr.op, expr.left, expr.right, env);
      case "conditional": {
        const test = this.evaluate(expr.test, env);
        if (test.tag === "error") {
          return test;
        }
        return this.evaluate(test.value === true ? expr.consequent : expr.alternative, env);
      }
      case "assign":
        return this.evalAssignment(expr, env);
      case "arrow": {
        const { params, rest } = splitParams(expr.params);
        return ok(makeClosure(params, rest, expr.body, env));
      }
      case "call":
        return this.evalCall(expr.callee, expr.args, env);
      case "member": {
        const object = this.evaluate(expr.object, env);
        return object.tag === "error" ? object : this.memberGet(object.value, expr.name);
      }
      case "index": {
        const object = this.evaluate(expr.object, env);
        if (object.tag === "error") {
          return object;
        }
        const index = this.evaluate(expr.index, env);
        return index.tag === "error" ? index : this.indexGet(object.value, index.value);
      }
      case "new-error": {
        const args = this.evalExprList(expr.args, env);
        if (args.tag === "error") {
          return fail(args.error);
        }
        const message = args.values[0];
        return ok(makeErrorValue(message === undefined ? "" : this.render(message)));
      }
      case "new-map":
        return this.evalNewMap(expr.args, env);
      case "new-set": {
        const args = this.evalExprList(expr.args, env);
        if (args.tag === "error") {
          return fail(args.error);
        }
        const items = args.values[0];
        if (items === undefined) {
          return ok(makeSet());
        }
        if (!isArrayValue(items)) {
          return bad("new Set", "argument is not an array");
        }
        return ok(makeSet(items.items));
      }
      case "delay":
        return ok(new ThunkValue(expr.expr, env));
      case "force":
        return this.evalForce(expr.expr, env);
      case "require":
      case "choose":
      case "ramb":
      case "permanent-assign":
      case "if-fail":
        return fail({
          tag: "unknown-syntax",
          construct: "search-experiment (run through runAmbSource)",
        });
    }
  }

  evalTemplate(chunks: ReadonlyArray<string>, exprs: ReadonlyArray<Expr>, env: Env): Outcome {
    let text = "";
    for (let i = 0; i < chunks.length; i += 1) {
      text += chunks[i] ?? "";
      const inner = exprs[i];
      if (inner === undefined) {
        continue;
      }
      const value = this.evaluate(inner, env);
      if (value.tag === "error") {
        return value;
      }
      text += this.render(value.value);
    }
    return ok(text);
  }

  evalArray(elements: ReadonlyArray<Arg>, env: Env): Outcome {
    const items: Value[] = [];
    for (const element of elements) {
      const value = this.evaluate(element.expr, env);
      if (value.tag === "error") {
        return value;
      }
      if (element.kind === "spread") {
        if (!isArrayValue(value.value)) {
          return bad("spread", "spread argument is not an array");
        }
        items.push(...value.value.items);
        continue;
      }
      items.push(value.value);
    }
    return ok(makeArray(items));
  }

  evalObject(fields: ReadonlyArray<{ key: string; value: Expr }>, env: Env): Outcome {
    const entries: Array<readonly [string, Value]> = [];
    for (const field of fields) {
      const value = this.evaluate(field.value, env);
      if (value.tag === "error") {
        return value;
      }
      entries.push([field.key, value.value]);
    }
    return ok(makeRecord(entries));
  }

  evalUnary(op: "!" | "+" | "-" | "typeof", operand: Expr, env: Env): Outcome {
    const value = this.evaluate(operand, env);
    return value.tag === "error" ? value : applyUnaryOperation(op, value.value);
  }

  evalBinary(
    op: "+" | "-" | "*" | "/" | "%" | "<" | "<=" | ">" | ">=" | "===" | "!==",
    left: Expr,
    right: Expr,
    env: Env,
  ): Outcome {
    const first = this.evaluate(left, env);
    if (first.tag === "error") {
      return first;
    }
    const second = this.evaluate(right, env);
    if (second.tag === "error") {
      return second;
    }
    return applyBinaryOperation(op, first.value, second.value);
  }

  evalLogical(op: "&&" | "||", left: Expr, right: Expr, env: Env): Outcome {
    const first = this.evaluate(left, env);
    if (first.tag === "error") {
      return first;
    }
    if (typeof first.value !== "boolean") {
      return bad(op, "left operand is not a boolean");
    }
    const shortCircuits = op === "&&" ? !first.value : first.value;
    if (shortCircuits) {
      return ok(first.value);
    }
    const second = this.evaluate(right, env);
    if (second.tag === "error") {
      return second;
    }
    return typeof second.value === "boolean"
      ? ok(second.value)
      : bad(op, "right operand is not a boolean");
  }

  /** A simple assignment: target reference first, then the right-hand side. */
  evalAssignment(expr: Extract<Expr, { tag: "assign" }>, env: Env): Outcome {
    const target = expr.target;
    if (target.tag === "variable") {
      const cell = findCell(env, target.name);
      if (cell === undefined) {
        return fail({ tag: "unbound-name", name: target.name });
      }
      if (!cell.mutable) {
        return bad("=", "assignment to a const binding");
      }
      const value = this.evaluate(expr.value, env);
      if (value.tag === "error") {
        return value;
      }
      cell.value = value.value;
      cell.initialized = true;
      return ok(value.value);
    }
    if (target.tag === "member") {
      const object = this.evaluate(target.object, env);
      if (object.tag === "error") {
        return object;
      }
      const value = this.evaluate(expr.value, env);
      return value.tag === "error" ? value : this.memberSet(object.value, target.name, value.value);
    }
    if (target.tag === "index") {
      const object = this.evaluate(target.object, env);
      if (object.tag === "error") {
        return object;
      }
      const index = this.evaluate(target.index, env);
      if (index.tag === "error") {
        return index;
      }
      const value = this.evaluate(expr.value, env);
      return value.tag === "error" ? value : this.indexSet(object.value, index.value, value.value);
    }
    return fail({ tag: "unknown-syntax", construct: "assignment-target" });
  }

  evalArgs(args: ReadonlyArray<Arg>, env: Env): ArgsOutcome {
    const values: Value[] = [];
    for (const arg of args) {
      const value = this.evaluate(arg.expr, env);
      if (value.tag === "error") {
        return { tag: "error", error: value.error };
      }
      if (arg.kind === "spread") {
        if (!isArrayValue(value.value)) {
          return {
            tag: "error",
            error: {
              tag: "bad-operand",
              operator: "spread",
              detail: "spread argument is not an array",
            },
          };
        }
        values.push(...value.value.items);
        continue;
      }
      values.push(value.value);
    }
    return { tag: "args", values };
  }

  evalExprList(exprs: ReadonlyArray<Expr>, env: Env): ArgsOutcome {
    return this.evalArgs(
      exprs.map((expr): Arg => ({ kind: "item", expr })),
      env,
    );
  }

  evalNewMap(args: ReadonlyArray<Expr>, env: Env): Outcome {
    const evaluated = this.evalExprList(args, env);
    if (evaluated.tag === "error") {
      return fail(evaluated.error);
    }
    const entriesValue = evaluated.values[0];
    if (entriesValue === undefined) {
      return ok(makeMap());
    }
    if (!isArrayValue(entriesValue)) {
      return bad("new Map", "argument is not an entries array");
    }
    const entries: Array<readonly [Value, Value]> = [];
    for (const pair of entriesValue.items) {
      if (!isArrayValue(pair) || pair.items.length !== 2) {
        return bad("new Map", "entry is not a two-element array");
      }
      const key = pair.items[0];
      const value = pair.items[1];
      entries.push([key, value]);
    }
    return ok(makeMap(entries));
  }

  evalForce(expr: Expr, env: Env): Outcome {
    const value = this.evaluate(expr, env);
    if (value.tag === "error") {
      return value;
    }
    if (!isThunkValue(value.value)) {
      return bad("force", "argument is not a thunk");
    }
    const thunk = value.value;
    if (this.mode === "lazy-memoized-experiment" && thunk.evaluated) {
      return ok(thunk.computed);
    }
    this.evaluations += 1;
    const computed = this.evaluate(thunk.expr, thunk.env);
    if (computed.tag === "error") {
      return computed;
    }
    if (this.mode === "lazy-memoized-experiment") {
      thunk.computed = computed.value;
      thunk.evaluated = true;
    }
    return ok(computed.value);
  }

  // ------------------------------------------------------------------
  // Names, members, and indices
  // ------------------------------------------------------------------

  lookupVariableValue(name: string, env: Env): Outcome {
    const cell = findCell(env, name);
    if (cell === undefined) {
      return fail({ tag: "unbound-name", name });
    }
    if (!cell.initialized) {
      return fail({ tag: "tdz-access", name });
    }
    return ok(cell.value);
  }

  setVariableValue(name: string, value: Value, env: Env): Outcome {
    const cell = findCell(env, name);
    if (cell === undefined) {
      return fail({ tag: "unbound-name", name });
    }
    if (!cell.mutable) {
      return bad("=", "assignment to a const binding");
    }
    cell.value = value;
    cell.initialized = true;
    return ok(value);
  }

  defineVariableValue(name: string, value: Value, env: Env): void {
    env.bindings.set(name, makeCell(value, true));
  }

  memberGet(object: Value, name: string): Outcome {
    if (typeof object === "string") {
      return name === "length" ? ok(object.length) : fail({ tag: "unknown-field", field: name });
    }
    if (isArrayValue(object)) {
      return name === "length"
        ? ok(object.items.length)
        : fail({ tag: "unknown-field", field: name });
    }
    if (isMapValue(object)) {
      return name === "size"
        ? ok(object.entries.size)
        : fail({ tag: "unknown-field", field: name });
    }
    if (isSetValue(object)) {
      return name === "size" ? ok(object.items.size) : fail({ tag: "unknown-field", field: name });
    }
    if (isErrorValue(object)) {
      return name === "message" ? ok(object.message) : fail({ tag: "unknown-field", field: name });
    }
    if (isRecordValue(object)) {
      return ok(object.fields.get(name));
    }
    return fail({ tag: "unknown-field", field: name });
  }

  memberSet(object: Value, name: string, value: Value): Outcome {
    if (!isRecordValue(object)) {
      return bad("assign", "member target is not a record");
    }
    if (object.readonlyFields.has(name)) {
      return fail({ tag: "readonly-field", field: name });
    }
    object.fields.set(name, value);
    return ok(value);
  }

  indexGet(object: Value, index: Value): Outcome {
    if (isArrayValue(object) && typeof index === "number") {
      return ok(itemAt(object.items, index));
    }
    if (typeof object === "string" && typeof index === "number") {
      return ok(itemAt([...object], index));
    }
    if (isRecordValue(object) && typeof index === "string") {
      return ok(object.fields.get(index));
    }
    if (isMapValue(object)) {
      return ok(object.entries.get(index));
    }
    return bad("index", "receiver or index has the wrong kind");
  }

  indexSet(object: Value, index: Value, value: Value): Outcome {
    if (isArrayValue(object) && typeof index === "number") {
      if (!Number.isInteger(index) || index < 0) {
        return bad("index", "array index is not a non-negative integer");
      }
      while (object.items.length < index) {
        object.items.push(undefined);
      }
      object.items[index] = value;
      return ok(value);
    }
    if (isRecordValue(object) && typeof index === "string") {
      return this.memberSet(object, index, value);
    }
    if (isMapValue(object)) {
      object.entries.set(index, value);
      return ok(value);
    }
    return bad("index", "receiver or index has the wrong kind");
  }

  // ------------------------------------------------------------------
  // Application
  // ------------------------------------------------------------------

  evalCall(callee: Expr, args: ReadonlyArray<Arg>, env: Env): Outcome {
    if (
      callee.tag === "member" &&
      callee.object.tag === "variable" &&
      callee.object.name === "console" &&
      callee.name === "log"
    ) {
      return this.emitOutput(args, env);
    }
    if (callee.tag === "member") {
      const receiver = this.evaluate(callee.object, env);
      if (receiver.tag === "error") {
        return receiver;
      }
      const values = this.evalArgs(args, env);
      if (values.tag === "error") {
        return fail(values.error);
      }
      return this.callMember(receiver.value, callee.name, values.values);
    }
    const target = this.evaluate(callee, env);
    if (target.tag === "error") {
      return target;
    }
    const values = this.evalArgs(args, env);
    if (values.tag === "error") {
      return fail(values.error);
    }
    return this.applyProcedure(target.value, values.values);
  }

  /** The admitted output operation: one argument, appended to the transcript. */
  emitOutput(args: ReadonlyArray<Arg>, env: Env): Outcome {
    const values = this.evalArgs(args, env);
    if (values.tag === "error") {
      return fail(values.error);
    }
    if (values.values.length !== 1) {
      return fail({
        tag: "bad-operand",
        operator: "console.log",
        detail: "expected exactly one argument",
      });
    }
    this.transcript.push(this.render(values.values[0]));
    return ok(undefined);
  }

  applyProcedure(procedure: Value, args: ReadonlyArray<Value>): Outcome {
    if (isPrimitive(procedure)) {
      return procedure.fn(args);
    }
    if (!isClosure(procedure)) {
      return fail({ tag: "not-callable", detail: renderShallow(procedure) });
    }
    return this.callClosure(procedure, args);
  }

  callClosure(closure: Closure, args: ReadonlyArray<Value>): Outcome {
    const required = closure.params.length;
    const fits = closure.rest === null ? args.length === required : args.length >= required;
    if (!fits) {
      return fail({ tag: "wrong-arity", expected: required, given: args.length });
    }
    const frame = child(closure.env);
    for (let i = 0; i < required; i += 1) {
      const name = closure.params[i];
      if (name !== undefined) {
        frame.bindings.set(name, makeCell(args[i], true));
      }
    }
    if (closure.rest !== null) {
      frame.bindings.set(closure.rest, makeCell(makeArray(args.slice(required)), true));
    }
    return completionToOutcome(this.execSequence(closure.body.body, frame));
  }

  callMember(receiver: Value, name: string, args: ReadonlyArray<Value>): Outcome {
    if (isArrayValue(receiver) && CALLBACK_METHODS[name] === true) {
      return this.callArrayCallback(receiver, name, args);
    }
    const table = tableKeyOf(receiver, name);
    const builtin = table === undefined ? undefined : this.#builtins.get(table);
    if (builtin !== undefined) {
      return builtin([receiver, ...args]);
    }
    if (isRecordValue(receiver)) {
      const field = receiver.fields.get(name);
      if (field === undefined) {
        return fail({ tag: "not-callable", detail: `missing field ${name}` });
      }
      return this.applyProcedure(field, args);
    }
    return fail({ tag: "not-callable", detail: `${kindOf(receiver)}.${name}` });
  }

  callArrayCallback(receiver: ArrayValue, name: string, args: ReadonlyArray<Value>): Outcome {
    const fn = args[0];
    if (fn === undefined || (!isClosure(fn) && !isPrimitive(fn))) {
      return fail({ tag: "not-callable", detail: `${name} needs a procedure` });
    }
    const call = (callArgs: ReadonlyArray<Value>): Outcome => this.applyProcedure(fn, callArgs);
    const items = receiver.items;
    if (name === "forEach") {
      for (const item of items) {
        const result = call([item]);
        if (result.tag === "error") {
          return result;
        }
      }
      return ok(undefined);
    }
    if (name === "map" || name === "flatMap" || name === "filter") {
      const out: Value[] = [];
      for (const item of items) {
        const result = call([item]);
        if (result.tag === "error") {
          return result;
        }
        if (name === "filter") {
          if (result.value === true) {
            out.push(item);
          }
          continue;
        }
        if (name === "flatMap" && isArrayValue(result.value)) {
          out.push(...result.value.items);
          continue;
        }
        out.push(result.value);
      }
      return ok(makeArray(out));
    }
    if (name === "find" || name === "some" || name === "every") {
      for (const item of items) {
        const result = call([item]);
        if (result.tag === "error") {
          return result;
        }
        const holds = result.value === true;
        if (name === "find" && holds) {
          return ok(item);
        }
        if (name === "some" && holds) {
          return ok(true);
        }
        if (name === "every" && !holds) {
          return ok(false);
        }
      }
      return ok(name === "find" ? undefined : name === "every");
    }
    return this.reduceItems(items, name, args, call);
  }

  reduceItems(
    items: ReadonlyArray<Value>,
    name: string,
    args: ReadonlyArray<Value>,
    call: (callArgs: ReadonlyArray<Value>) => Outcome,
  ): Outcome {
    const right = name === "reduceRight";
    const ordered = right ? [...items].reverse() : [...items];
    const hasInit = args.length > 1;
    if (!hasInit && ordered.length === 0) {
      return bad(name, "empty array without an initial value");
    }
    let acc = hasInit ? args[1] : ordered[0];
    for (const item of ordered.slice(hasInit ? 0 : 1)) {
      const result = call([acc, item]);
      if (result.tag === "error") {
        return result;
      }
      acc = result.value;
    }
    return ok(acc);
  }

  // ------------------------------------------------------------------
  // Statements
  // ------------------------------------------------------------------

  execStatement(stmt: Stmt, env: Env): Completion {
    switch (stmt.tag) {
      case "block":
        return this.execBody(stmt.body, env);
      case "if": {
        const test = this.evaluate(stmt.test, env);
        if (test.tag === "error") {
          return { tag: "error", error: test.error };
        }
        return this.execStatement(
          test.value === true ? stmt.consequent : (stmt.alternative ?? emptyBlock()),
          env,
        );
      }
      case "while": {
        for (;;) {
          const test = this.evaluate(stmt.test, env);
          if (test.tag === "error") {
            return { tag: "error", error: test.error };
          }
          if (test.value !== true) {
            return normal(undefined);
          }
          const body = this.execStatement(stmt.body, env);
          if (body.tag === "break") {
            return normal(undefined);
          }
          if (body.tag === "return" || body.tag === "throw" || body.tag === "error") {
            return body;
          }
        }
      }
      case "for-of":
        return this.execForOf(stmt.name, stmt.iterable, stmt.body, env);
      case "switch":
        return this.execSwitch(stmt, env);
      case "return": {
        if (stmt.argument === null) {
          return { tag: "return", value: undefined };
        }
        const value = this.evaluate(stmt.argument, env);
        return value.tag === "error"
          ? { tag: "error", error: value.error }
          : { tag: "return", value: value.value };
      }
      case "break":
        return { tag: "break" };
      case "continue":
        return { tag: "continue" };
      case "throw": {
        const value = this.evaluate(stmt.argument, env);
        return value.tag === "error"
          ? { tag: "error", error: value.error }
          : { tag: "throw", value: value.value };
      }
      case "try":
        return this.execTry(stmt, env);
      case "expr-stmt": {
        const value = this.evaluate(stmt.expr, env);
        return value.tag === "error" ? { tag: "error", error: value.error } : normal(value.value);
      }
    }
  }

  execForOf(name: string, iterable: Expr, body: Stmt, env: Env): Completion {
    const value = this.evaluate(iterable, env);
    if (value.tag === "error") {
      return { tag: "error", error: value.error };
    }
    if (!isArrayValue(value.value)) {
      return {
        tag: "error",
        error: { tag: "bad-operand", operator: "for-of", detail: "iterable is not an array" },
      };
    }
    for (const item of value.value.items) {
      const frame = child(env);
      frame.bindings.set(name, makeCell(item, true));
      const completion = this.execStatement(body, frame);
      if (completion.tag === "break") {
        return normal(undefined);
      }
      if (completion.tag === "return" || completion.tag === "throw" || completion.tag === "error") {
        return completion;
      }
    }
    return normal(undefined);
  }

  execSwitch(stmt: Extract<Stmt, { tag: "switch" }>, env: Env): Completion {
    const disc = this.evaluate(stmt.discriminant, env);
    if (disc.tag === "error") {
      return { tag: "error", error: disc.error };
    }
    const items: Array<Decl | Stmt> = [];
    for (const clause of stmt.cases) {
      items.push(...clause.body);
    }
    if (stmt.defaultBody !== null) {
      items.push(...stmt.defaultBody);
    }
    const frame = child(env);
    this.predeclare(items, frame);
    for (const clause of stmt.cases) {
      const test = this.evaluate(clause.test, env);
      if (test.tag === "error") {
        return { tag: "error", error: test.error };
      }
      if (test.value === disc.value) {
        return this.asSwitchBody(clause.body, frame);
      }
    }
    return stmt.defaultBody === null
      ? normal(undefined)
      : this.asSwitchBody(stmt.defaultBody, frame);
  }

  asSwitchBody(items: ReadonlyArray<Decl | Stmt>, frame: Env): Completion {
    const completion = this.execSequence(items, frame);
    return completion.tag === "break" ? normal(undefined) : completion;
  }

  execTry(stmt: Extract<Stmt, { tag: "try" }>, env: Env): Completion {
    const body = this.execBody(stmt.block.body, env);
    let through: Completion = body;
    const thrown: { caught: true; value: Value } | { caught: false } =
      body.tag === "throw"
        ? { caught: true, value: body.value }
        : body.tag === "error" && body.error.tag === "guest-throw"
          ? { caught: true, value: body.error.value }
          : { caught: false };
    if (thrown.caught && stmt.handler !== null) {
      const frame = child(env);
      if (stmt.handler.param !== null) {
        frame.bindings.set(stmt.handler.param, makeCell(thrown.value, true));
      }
      through = this.execBody(stmt.handler.body.body, frame);
    }
    if (stmt.finalizer === null) {
      return through;
    }
    const finishing = this.execBody(stmt.finalizer.body, env);
    return finishing.tag === "normal" ? through : finishing;
  }

  predeclare(items: ReadonlyArray<Decl | Stmt>, frame: Env): void {
    for (const item of items) {
      if (item.tag === "var-decl") {
        frame.bindings.set(item.name, makeCell(undefined, false, item.kind === "let"));
        continue;
      }
      if (item.tag === "function-decl") {
        const { params, rest } = splitParams(item.params);
        const closure = makeClosure(params, rest, item.body, frame);
        frame.bindings.set(item.name, makeCell(closure, true));
      }
    }
  }

  execBody(items: ReadonlyArray<Decl | Stmt>, env: Env): Completion {
    return this.execSequence(items, child(env));
  }

  execSequence(items: ReadonlyArray<Decl | Stmt>, frame: Env): Completion {
    this.predeclare(items, frame);
    let last: Value;
    for (const item of items) {
      const completion = this.execItem(item, frame);
      if (completion.tag !== "normal") {
        return completion;
      }
      last = completion.value;
    }
    return normal(last);
  }

  execItem(item: Decl | Stmt, frame: Env): Completion {
    if (item.tag === "var-decl") {
      const value = this.evaluate(item.init, frame);
      if (value.tag === "error") {
        return { tag: "error", error: value.error };
      }
      const cell = findCell(frame, item.name);
      if (cell === undefined) {
        frame.bindings.set(item.name, makeCell(value.value, true));
      } else {
        cell.value = value.value;
        cell.initialized = true;
      }
      return normal(value.value);
    }
    if (item.tag === "function-decl") {
      const { params, rest } = splitParams(item.params);
      const closure = makeClosure(params, rest, item.body, frame);
      frame.bindings.set(item.name, makeCell(closure, true));
      return normal(undefined);
    }
    if (item.tag === "type-decl" || item.tag === "interface-decl" || item.tag === "import") {
      return normal(undefined);
    }
    return this.execStatement(item, frame);
  }

  render(value: Value): string {
    if (typeof value === "string") {
      return value;
    }
    if (value === null) {
      return "null";
    }
    if (value === undefined) {
      return "undefined";
    }
    if (typeof value === "number" || typeof value === "boolean") {
      return String(value);
    }
    if (isArrayValue(value)) {
      return `[${value.items.map((item) => this.render(item)).join(", ")}]`;
    }
    return renderShallow(value);
  }
}

// ---------------------------------------------------------------------
// Pure helpers
// ---------------------------------------------------------------------

const emptyBlock = (): Stmt => ({
  tag: "block",
  body: [],
  span: { start: 0, end: 0, line: 1, column: 1 },
});

const itemAt = (items: ReadonlyArray<Value>, index: number): Value =>
  Number.isInteger(index) && index >= 0 && index < items.length ? items[index] : undefined;

const kindOf = (value: Value): string => {
  if (value === null) {
    return "object";
  }
  if (isClosure(value) || isPrimitive(value)) {
    return "function";
  }
  return typeof value;
};

const renderShallow = (value: Value): string => {
  if (typeof value === "string") {
    return value;
  }
  return String(value ?? "undefined");
};

/** Splits a parameter list into positional names and one rest name. */
export const splitParams = (
  params: ReadonlyArray<Param>,
): { params: ReadonlyArray<string>; rest: string | null } => {
  const names: string[] = [];
  let rest: string | null = null;
  for (const entry of params) {
    if (entry.kind === "rest") {
      rest = entry.name;
      continue;
    }
    names.push(entry.name);
  }
  return { params: names, rest };
};

/** Applies a unary operation to an evaluated operand. */
export const applyUnaryOperation = (op: "!" | "+" | "-" | "typeof", value: Value): Outcome => {
  if (op === "typeof") {
    return ok(kindOf(value));
  }
  if (op === "!") {
    return typeof value === "boolean" ? ok(!value) : bad("!", "operand is not a boolean");
  }
  return typeof value === "number"
    ? ok(op === "+" ? +value : -value)
    : bad(op, "operand is not a number");
};

/** Applies a binary operation to two evaluated operands. */
export const applyBinaryOperation = (
  op: "+" | "-" | "*" | "/" | "%" | "<" | "<=" | ">" | ">=" | "===" | "!==",
  left: Value,
  right: Value,
): Outcome => {
  if (op === "===" || op === "!==") {
    const same = left === right;
    return ok(op === "===" ? same : !same);
  }
  if (op === "+") {
    if (typeof left === "number" && typeof right === "number") {
      return ok(left + right);
    }
    if (typeof left === "string" && typeof right === "string") {
      return ok(left + right);
    }
    return bad("+", "operands are not both numbers or both strings");
  }
  if (typeof left !== "number" || typeof right !== "number") {
    return bad(op, "operands are not numbers");
  }
  switch (op) {
    case "-":
      return ok(left - right);
    case "*":
      return ok(left * right);
    case "/":
      return ok(left / right);
    case "%":
      return ok(left % right);
    case "<":
      return ok(left < right);
    case "<=":
      return ok(left <= right);
    case ">":
      return ok(left > right);
    default:
      return ok(left >= right);
  }
};

const tableKeyOf = (receiver: Value, name: string): string | undefined => {
  if (isArrayValue(receiver)) {
    return `array.${name}`;
  }
  if (typeof receiver === "string") {
    return `string.${name}`;
  }
  if (isMapValue(receiver)) {
    return `map.${name}`;
  }
  if (isSetValue(receiver)) {
    return `set.${name}`;
  }
  return undefined;
};

// ---------------------------------------------------------------------
// Public engine API
// ---------------------------------------------------------------------

/** The book's `eval`: one case analysis over the checked syntax. */
export const evaluate = (expr: Expr, env: Env): Outcome => new Session("core").evaluate(expr, env);

/** The book's `list-of-values`: operands evaluated left to right. */
export const listOfValues = (exps: ReadonlyArray<Expr>, env: Env): Outcome => {
  const session = new Session("core");
  return session.evalArray(
    exps.map((expr): Arg => ({ kind: "item", expr })),
    env,
  );
};

/** The book's `eval-if` over the shared `if` node, preserving completion. */
export const evalIf = (stmt: Extract<Stmt, { tag: "if" }>, env: Env): Completion =>
  new Session("core").execStatement(stmt, env);

/** The book's `eval-sequence`: every form but the last for effect. */
export const evalSequence = (items: ReadonlyArray<Decl | Stmt>, env: Env): Completion =>
  new Session("core").execSequence(items, env);

/** The book's `eval-declaration`: bind the declaration in the frame. */
export const evalDeclaration = (decl: Decl, env: Env): Completion =>
  new Session("core").execItem(decl, env);

/** The book's `eval-assignment`: compute, then write the found cell. */
export const evalAssignment = (expr: Expr, env: Env): Outcome =>
  expr.tag === "assign"
    ? new Session("core").evalAssignment(expr, env)
    : fail({ tag: "unknown-syntax", construct: "eval-assignment expects an assign node" });

/** The book's `make-procedure`: packages a closure over its environment. */
export const makeProcedure = (params: ReadonlyArray<string>, body: Program, env: Env): Closure => {
  const span = body[0]?.span ?? { start: 0, end: 0, line: 1, column: 1 };
  return makeClosure(params, null, { body, span }, env);
};

/** The book's `extend-environment`: one frame binding names to values. */
export const extendEnvironment = (
  names: ReadonlyArray<string>,
  args: ReadonlyArray<Value>,
  base: Env,
): EnvOutcome => {
  if (names.length !== args.length) {
    return {
      tag: "error",
      error: { tag: "wrong-arity", expected: names.length, given: args.length },
    };
  }
  const frame = child(base);
  for (let i = 0; i < names.length; i += 1) {
    const name = names[i];
    if (name !== undefined) {
      frame.bindings.set(name, makeCell(args[i], true));
    }
  }
  return { tag: "ok", env: frame };
};

/** The book's `lookup-variable-value`. */
export const lookupVariableValue = (name: string, env: Env): Outcome =>
  new Session("core").lookupVariableValue(name, env);

/** The book's `set-variable-value!`: write the cell the chain finds. */
export const setVariableValue = (name: string, value: Value, env: Env): Outcome =>
  new Session("core").setVariableValue(name, value, env);

/** The book's `define-variable!`: bind in this frame, shadowing outer ones. */
export const defineVariableValue = (name: string, value: Value, env: Env): void =>
  new Session("core").defineVariableValue(name, value, env);

/** A fresh global environment with the builtin and namespace bindings. */
export const globalEnvironment = (): Env => new Session("core").globalEnv();

/** Maps a statement completion to the evaluator outcome channel. */
const completionToOutcome = (completion: Completion): Outcome => {
  switch (completion.tag) {
    case "normal":
    case "return":
      return ok(completion.value);
    case "throw":
      return fail({ tag: "guest-throw", value: completion.value });
    case "error":
      return fail(completion.error);
    case "break":
    case "continue":
      return fail({ tag: "unknown-syntax", construct: `unexpected-${completion.tag}` });
  }
};

/** Runs an admitted program in the given session, mapping completions. */
export const executeProgram = (program: Program, session: Session): Outcome =>
  completionToOutcome(session.execBody(program, session.globalEnv()));

const runProgram = (program: Program, mode: ExperimentMode): RunResult => {
  const session = new Session(mode);
  return { outcome: executeProgram(program, session), transcript: session.transcript };
};

/** Reads, admits, and runs one source unit; no guest effect on rejection. */
export const runSource = (text: string, mode: ExperimentMode = "core"): RunResult => {
  const admission = admitSource(text, mode);
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
  return runProgram(admission.program, mode);
};

/** The book's driver loop over a finite REPL session. */
export const driverLoop = (env: Env, inputs: ReadonlyArray<string>): RunResult => {
  const session = new Session("core");
  const lines: string[] = [];
  let last: Outcome = ok(undefined);
  for (const input of inputs) {
    const admission = admitSource(input);
    if (!admission.ok) {
      return {
        outcome: fail({
          tag: "unknown-syntax",
          construct:
            admission.diagnostics[0]?.construct ?? `TS${admission.hostDiagnostics[0]?.code ?? 0}`,
        }),
        transcript: lines,
      };
    }
    lines.push(";;; M-Eval input:", input);
    const completion = session.execSequence(admission.program, env);
    last =
      completion.tag === "normal" || completion.tag === "return"
        ? ok(completion.value)
        : completion.tag === "throw"
          ? fail({ tag: "guest-throw", value: completion.value })
          : completion.tag === "error"
            ? fail(completion.error)
            : fail({ tag: "unknown-syntax", construct: "unexpected-break" });
    if (last.tag === "error") {
      lines.push(";;; M-Eval error:", JSON.stringify(last.error));
      return { outcome: last, transcript: lines };
    }
    lines.push(";;; M-Eval value:", format(last.value));
  }
  return { outcome: last, transcript: lines };
};

// ---------------------------------------------------------------------
// 4.1.7 Separating syntactic analysis from execution
// ---------------------------------------------------------------------

/** The book's `analyze`: syntax once, execution many times. */
export const analyze = (expr: Expr): ExecutionProcedure =>
  analyzedProcedure(expr, new Session("core"));

/** The analyzed evaluator's `eval`: analyze once, run once. */
export const evalAnalyzed = (expr: Expr, env: Env): Outcome => analyze(expr)(env);

/** Runs an admitted program through the analyzer over one shared session. */
export const runAnalyzedSource = (text: string, mode: ExperimentMode = "core"): RunResult => {
  const admission = admitSource(text, mode);
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
  const session = new Session(mode);
  const env = session.globalEnv();
  session.predeclare(admission.program, env);
  let outcome: Outcome = ok(undefined);
  for (const form of admission.program) {
    outcome =
      form.tag === "expr-stmt"
        ? analyzedProcedure(form.expr, session)(env)
        : completionToOutcome(session.execItem(form, env));
    if (outcome.tag === "error") {
      break;
    }
  }
  return { outcome, transcript: session.transcript };
};

const analyzedProcedure = (expr: Expr, session: Session): ExecutionProcedure => {
  switch (expr.tag) {
    case "number":
    case "string":
    case "boolean":
      return () => ok(expr.value);
    case "null":
      return () => ok(null);
    case "undefined":
      return () => ok(undefined);
    case "variable":
      return (env) => session.lookupVariableValue(expr.name, env);
    case "binary": {
      const left = analyzedProcedure(expr.left, session);
      const right = analyzedProcedure(expr.right, session);
      return (env) => {
        const first = left(env);
        if (first.tag === "error") {
          return first;
        }
        const second = right(env);
        return second.tag === "error"
          ? second
          : applyBinaryOperation(expr.op, first.value, second.value);
      };
    }
    case "logical": {
      const left = analyzedProcedure(expr.left, session);
      const right = analyzedProcedure(expr.right, session);
      return (env) => {
        const first = left(env);
        if (first.tag === "error") {
          return first;
        }
        if (typeof first.value !== "boolean") {
          return bad(expr.op, "left operand is not a boolean");
        }
        const shortCircuits = expr.op === "&&" ? !first.value : first.value;
        if (shortCircuits) {
          return ok(first.value);
        }
        const second = right(env);
        if (second.tag === "error") {
          return second;
        }
        return typeof second.value === "boolean"
          ? ok(second.value)
          : bad(expr.op, "right operand is not a boolean");
      };
    }
    case "conditional": {
      const test = analyzedProcedure(expr.test, session);
      const consequent = analyzedProcedure(expr.consequent, session);
      const alternative = analyzedProcedure(expr.alternative, session);
      return (env) => {
        const holds = test(env);
        if (holds.tag === "error") {
          return holds;
        }
        return (holds.value === true ? consequent : alternative)(env);
      };
    }
    case "arrow": {
      const { params, rest } = splitParams(expr.params);
      return (env) => ok(makeClosure(params, rest, expr.body, env));
    }
    case "call": {
      // Member calls (the `console.log` boundary and host methods) and spread
      // arguments carry the evaluator's own call semantics; only plain
      // procedure applications are pre-analyzed here.
      if (expr.callee.tag === "member" || expr.args.some((arg) => arg.kind !== "item")) {
        return (env) => session.evalCall(expr.callee, expr.args, env);
      }
      const callee = analyzedProcedure(expr.callee, session);
      const args = expr.args.map((arg) => analyzedProcedure(arg.expr, session));
      return (env) => {
        const target = callee(env);
        if (target.tag === "error") {
          return target;
        }
        const values: Value[] = [];
        for (const arg of args) {
          const value = arg(env);
          if (value.tag === "error") {
            return value;
          }
          values.push(value.value);
        }
        return session.applyProcedure(target.value, values);
      };
    }
    default:
      return (env) => session.evaluate(expr, env);
  }
};
