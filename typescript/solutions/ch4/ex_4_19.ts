// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.19: the internal definition scoping debate. One body, three
 * rules. Sequential keeps the engine's own application; the guest
 * runtime pre-binds internal names as uninitialized cells, so this
 * reading settles on Alyssa's side: the sibling read faults with TDZ
 * instead of silently seeing an outer binding. Scanned is the 4.16
 * mechanism and answers the same fault. Eva's rule is simultaneous
 * definitions whose value expressions see each other's final values:
 * every internal name is pre-bound uninitialized, and reading one forces
 * that name's value expression in the current frame and memoizes the
 * value by writing it back; forcing a name already being forced is a
 * circular-definition error. The forcing reader covers the expression
 * forms a value expression can use; statement forms inside value
 * expressions are outside the rule's shape.
 */
import {
  applyBinaryOperation,
  applyUnaryOperation,
  extendEnvironment,
  Session,
  setVariableValue,
} from "../../packages/ch4/src/01-metacircular.js";
import { type Env, makeCell } from "../../packages/ch4/src/runtime/env.js";
import type { Completion, Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { fail, failed, normal, ok, outcomeOf } from "../../packages/ch4/src/runtime/errors.js";
import {
  isClosure,
  isPrimitive,
  makeArray,
  makeClosure,
  type Value,
} from "../../packages/ch4/src/runtime/value.js";
import {
  assign,
  type Decl,
  type Expr,
  exprStmt,
  ident,
  type Stmt,
} from "../../packages/ch4/src/syntax/ast.js";
import { addUninitialized, scanOutDefinitions } from "./ex_4_16.js";

/** The simultaneous rule's forcing state: value expressions and memo. */
interface Forcing {
  readonly values: Map<string, Expr>;
  readonly progress: Set<string>;
  readonly session: Session;
}

const forceName = (name: string, env: Env, forcing: Forcing): Outcome => {
  const cell = env.bindings.get(name);
  if (cell === undefined) {
    return fail({ tag: "unbound-name", name });
  }
  if (cell.initialized) {
    return ok(cell.value);
  }
  if (forcing.progress.has(name)) {
    return fail({ tag: "bad-operand", operator: "letrec", detail: `circular definition: ${name}` });
  }
  const value = forcing.values.get(name);
  if (value === undefined) {
    return fail({ tag: "tdz-access", name });
  }
  forcing.progress.add(name);
  const computed = evalForcing(value, env, forcing);
  forcing.progress.delete(name);
  if (computed.tag === "error") {
    return computed;
  }
  cell.value = computed.value;
  cell.initialized = true;
  return ok(computed.value);
};

const applyForced = (
  procedure: Value,
  args: ReadonlyArray<Value>,
  env: Env,
  forcing: Forcing,
): Outcome => {
  if (isPrimitive(procedure)) {
    return procedure.fn(args);
  }
  if (!isClosure(procedure)) {
    return fail({ tag: "not-callable", detail: "simultaneous rule: value is not a procedure" });
  }
  const extended = extendEnvironment(procedure.params, args, procedure.env);
  if (extended.tag === "error") {
    return fail(extended.error);
  }
  return outcomeOf(forcing.session.execSequence(procedure.body.body, extended.env));
};

/** The forcing reader: sibling reads force their value expression. */
export const evalForcing = (expr: Expr, env: Env, forcing: Forcing): Outcome => {
  switch (expr.tag) {
    case "number":
    case "string":
    case "boolean":
      return ok(expr.value);
    case "null":
      return ok(null);
    case "undefined":
      return ok(undefined);
    case "variable":
      return forcing.values.has(expr.name)
        ? forceName(expr.name, env, forcing)
        : forcing.session.lookupVariableValue(expr.name, env);
    case "binary": {
      const left = evalForcing(expr.left, env, forcing);
      if (left.tag === "error") {
        return left;
      }
      const right = evalForcing(expr.right, env, forcing);
      return right.tag === "error" ? right : applyBinaryOperation(expr.op, left.value, right.value);
    }
    case "unary": {
      const operand = evalForcing(expr.operand, env, forcing);
      return operand.tag === "error" ? operand : applyUnaryOperation(expr.op, operand.value);
    }
    case "logical": {
      const left = evalForcing(expr.left, env, forcing);
      if (left.tag === "error") {
        return left;
      }
      if (typeof left.value !== "boolean") {
        return fail({
          tag: "bad-operand",
          operator: expr.op,
          detail: "left operand is not a boolean",
        });
      }
      const shortCircuits = expr.op === "&&" ? !left.value : left.value;
      if (shortCircuits) {
        return ok(left.value);
      }
      const right = evalForcing(expr.right, env, forcing);
      if (right.tag === "error") {
        return right;
      }
      return typeof right.value === "boolean"
        ? ok(right.value)
        : fail({ tag: "bad-operand", operator: expr.op, detail: "right operand is not a boolean" });
    }
    case "conditional": {
      const test = evalForcing(expr.test, env, forcing);
      if (test.tag === "error") {
        return test;
      }
      return evalForcing(test.value === true ? expr.consequent : expr.alternative, env, forcing);
    }
    case "assign": {
      if (expr.target.tag !== "variable") {
        return forcing.session.evaluate(expr, env);
      }
      const value = evalForcing(expr.value, env, forcing);
      return value.tag === "error" ? value : setVariableValue(expr.target.name, value.value, env);
    }
    case "arrow": {
      const names: string[] = [];
      let rest: string | null = null;
      for (const parameter of expr.params) {
        if (parameter.kind === "rest") {
          rest = parameter.name;
          continue;
        }
        names.push(parameter.name);
      }
      return ok(makeClosure(names, rest, expr.body, env));
    }
    case "call": {
      const procedure = evalForcing(expr.callee, env, forcing);
      if (procedure.tag === "error") {
        return procedure;
      }
      const args: Value[] = [];
      for (const arg of expr.args) {
        const value = evalForcing(arg.expr, env, forcing);
        if (value.tag === "error") {
          return value;
        }
        args.push(value.value);
      }
      return applyForced(procedure.value, args, env, forcing);
    }
    default:
      return forcing.session.evaluate(expr, env);
  }
};

/**
 * Eva's rule: simultaneous definitions with memoized forcing. Every
 * internal name is pre-bound uninitialized; each write's value
 * expression forces its siblings to their final values.
 */
export const execSimultaneous = (
  items: ReadonlyArray<Decl | Stmt>,
  env: Env,
  session: Session = new Session("core"),
): Completion => {
  const scanned = scanOutDefinitions(items);
  const values = new Map<string, Expr>();
  for (const item of scanned.body) {
    if (
      item.tag === "expr-stmt" &&
      item.expr.tag === "assign" &&
      item.expr.target.tag === "variable"
    ) {
      values.set(item.expr.target.name, item.expr.value);
    }
  }
  const forcing: Forcing = { values, progress: new Set(), session };
  for (const name of scanned.names) {
    addUninitialized(name, env);
  }
  const rest: Array<Decl | Stmt> = [];
  for (const item of scanned.body) {
    if (
      item.tag === "expr-stmt" &&
      item.expr.tag === "assign" &&
      item.expr.target.tag === "variable"
    ) {
      const forced = forceName(item.expr.target.name, env, forcing);
      if (forced.tag === "error") {
        return failed(forced.error);
      }
      continue;
    }
    rest.push(item);
  }
  return session.execSequence(rest, env);
};

/** Sequential application: the engine's own rule. */
export const applyRuleSequential = (
  items: ReadonlyArray<Decl | Stmt>,
  args: ReadonlyArray<Value>,
  params: ReadonlyArray<string>,
  env: Env,
  session: Session = new Session("core"),
): Outcome => {
  const extended = extendEnvironment(params, args, env);
  if (extended.tag === "error") {
    return fail(extended.error);
  }
  return outcomeOf(session.execSequence(items, extended.env));
};

export function ex_4_19(): string {
  return (
    "The guest runtime settles the debate on Alyssa's side: internal names are pre-bound " +
    "as uninitialized cells, so the sibling read in `const b = a + x; const a = 5;` faults " +
    "with tdz-access instead of silently answering 16, and the scanned rule answers the " +
    "same fault. Ben's 16 is unreachable in this guest — it needs sequential define " +
    "semantics the typed runtime excludes. Eva's rule answers 20: forcing a to its final " +
    "value 5 gives b = 15 and the answer a + b = 20. A force cycle is a circular-definition " +
    "error, not a hang."
  );
}
