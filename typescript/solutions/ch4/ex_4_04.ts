// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.4: variadic short-circuit `all` and `any` as special forms
 * next to the binary `logical` node. The extension nodes live beside
 * `logical` in the syntax data and are admitted only by this exercise's
 * own evaluator, exactly as the book's `and`/`or` are new special forms.
 * The pinned route is the special form: each operand is evaluated at
 * most once, operands must be boolean or the call is a `bad-operand`
 * fault, evaluation stops at the first decisive operand, and the result
 * is boolean — the same strict rule the guest `logical` node enforces.
 * `allToConditional` and `anyToConditional` keep the exercise's second
 * route: derivation over the conditional expression, whose evaluation
 * counts and fault behavior the tests compare against the special form.
 */
import {
  applyBinaryOperation,
  applyUnaryOperation,
  extendEnvironment,
  Session,
  setVariableValue,
  splitParams,
} from "../../packages/ch4/src/01-metacircular.js";
import { type Env, findCell, makeCell } from "../../packages/ch4/src/runtime/env.js";
import type { Completion, Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { fail, failed, normal, ok, outcomeOf } from "../../packages/ch4/src/runtime/errors.js";
import {
  isArrayValue,
  isClosure,
  isPrimitive,
  makeArray,
  makeClosure,
  makeErrorValue,
  makeMap,
  makeRecord,
  makeSet,
  ThunkValue,
  type Value,
} from "../../packages/ch4/src/runtime/value.js";
import { bool, cond, type Decl, type Expr, type Stmt } from "../../packages/ch4/src/syntax/ast.js";
import { noSpan, type Span } from "../../packages/ch4/src/syntax/diagnostics.js";

/** The variadic `all` special form: true when every operand is true. */
export interface AllNode {
  readonly tag: "all";
  readonly operands: ReadonlyArray<Expr>;
  readonly span: Span;
}

/** The variadic `any` special form: true when some operand is true. */
export interface AnyNode {
  readonly tag: "any";
  readonly operands: ReadonlyArray<Expr>;
  readonly span: Span;
}

/** The two extension nodes, beside the binary `logical` node. */
export type VariadicLogic = AllNode | AnyNode;

/** The syntax this exercise evaluates: the shared expressions plus its own forms. */
export type ExtendedExpr = Expr | VariadicLogic;

/** Builds the `all` form. */
export const allOf = (operands: ReadonlyArray<Expr>, span: Span = noSpan): AllNode => ({
  tag: "all",
  operands,
  span,
});

/** Builds the `any` form. */
export const anyOf = (operands: ReadonlyArray<Expr>, span: Span = noSpan): AnyNode => ({
  tag: "any",
  operands,
  span,
});

const bad = (operator: string, detail: string): Outcome =>
  fail({ tag: "bad-operand", operator, detail });

// ---------------------------------------------------------------------
// The special-form evaluator
// ---------------------------------------------------------------------

const applyForm = (procedure: Value, args: ReadonlyArray<Value>, session: Session): Outcome => {
  if (isClosure(procedure)) {
    const required = procedure.params.length;
    const bound = procedure.rest === null ? args : args.slice(0, required);
    const extended = extendEnvironment(procedure.params, bound, procedure.env);
    if (extended.tag === "error") {
      return fail(extended.error);
    }
    const frame = extended.env;
    if (procedure.rest !== null) {
      frame.bindings.set(procedure.rest, makeCell(makeArray(args.slice(required)), true));
    }
    return outcomeOf(execFormSequence(procedure.body.body, frame, session));
  }
  if (isPrimitive(procedure)) {
    return procedure.fn(args);
  }
  return fail({ tag: "not-callable", detail: "and/or evaluator: value is not a procedure" });
};

const execFormItem = (item: Decl | Stmt, env: Env, session: Session): Completion => {
  switch (item.tag) {
    case "var-decl": {
      const init = evalWithAllAny(item.init, env, session);
      if (init.tag === "error") {
        return failed(init.error);
      }
      const cell = env.bindings.get(item.name);
      if (cell === undefined) {
        env.bindings.set(item.name, makeCell(init.value, true, item.kind === "let"));
      } else {
        cell.value = init.value;
        cell.initialized = true;
      }
      return normal(init.value);
    }
    case "function-decl":
    case "import":
    case "type-decl":
    case "interface-decl":
      return normal(undefined);
    case "expr-stmt": {
      const value = evalWithAllAny(item.expr, env, session);
      return value.tag === "error" ? failed(value.error) : normal(value.value);
    }
    case "return": {
      if (item.argument === null) {
        return normal(undefined);
      }
      const value = evalWithAllAny(item.argument, env, session);
      return value.tag === "error" ? failed(value.error) : { tag: "return", value: value.value };
    }
    case "block":
      return execFormSequence(item.body, { bindings: new Map(), parent: env }, session);
    case "if": {
      const test = evalWithAllAny(item.test, env, session);
      if (test.tag === "error") {
        return failed(test.error);
      }
      const branch = test.value === true ? item.consequent : item.alternative;
      return branch === null ? normal(undefined) : execFormItem(branch, env, session);
    }
    default:
      return session.execStatement(item, env);
  }
};

const execFormSequence = (
  items: ReadonlyArray<Decl | Stmt>,
  env: Env,
  session: Session,
): Completion => {
  for (const item of items) {
    if (item.tag === "var-decl") {
      env.bindings.set(item.name, makeCell(undefined, false, item.kind === "let"));
    }
    if (item.tag === "function-decl") {
      const { params, required, rest } = splitParams(item.params);
      env.bindings.set(
        item.name,
        makeCell(makeClosure(params, required, rest, item.body, env), true),
      );
    }
  }
  let last: Completion = normal(undefined);
  for (const item of items) {
    last = execFormItem(item, env, session);
    if (last.tag !== "normal") {
      return last;
    }
  }
  return last;
};

/**
 * The special-form evaluator. `all` and `any` are its own cases: each
 * operand is evaluated at most once, must be boolean, and evaluation
 * stops at the first decisive operand. These extension nodes sit beside
 * the shared `Expr` union, so shared `Stmt` procedure bodies cannot embed
 * them; nested-body examples must use the derived conditional expressions.
 */
export const evalWithAllAny = (
  expr: ExtendedExpr,
  env: Env,
  session: Session = new Session("core"),
): Outcome => {
  switch (expr.tag) {
    case "all": {
      for (const operand of expr.operands) {
        const value = evalWithAllAny(operand, env, session);
        if (value.tag === "error") {
          return value;
        }
        if (typeof value.value !== "boolean") {
          return bad("all", "operand is not a boolean");
        }
        if (!value.value) {
          return ok(false);
        }
      }
      return ok(true);
    }
    case "any": {
      for (const operand of expr.operands) {
        const value = evalWithAllAny(operand, env, session);
        if (value.tag === "error") {
          return value;
        }
        if (typeof value.value !== "boolean") {
          return bad("any", "operand is not a boolean");
        }
        if (value.value) {
          return ok(true);
        }
      }
      return ok(false);
    }
    case "call": {
      const procedure = evalWithAllAny(expr.callee, env, session);
      if (procedure.tag === "error") {
        return procedure;
      }
      const args: Value[] = [];
      for (const arg of expr.args) {
        const value = evalWithAllAny(arg.expr, env, session);
        if (value.tag === "error") {
          return value;
        }
        if (arg.kind === "spread") {
          if (!isArrayValue(value.value)) {
            return bad("spread", "spread argument is not an array");
          }
          args.push(...value.value.items);
          continue;
        }
        args.push(value.value);
      }
      return applyForm(procedure.value, args, session);
    }
    case "assign": {
      const target = expr.target;
      if (target.tag === "variable") {
        const cell = findCell(env, target.name);
        if (cell === undefined) {
          return fail({ tag: "unbound-name", name: target.name });
        }
        const value = evalWithAllAny(expr.value, env, session);
        if (value.tag === "error") {
          return value;
        }
        return setVariableValue(target.name, value.value, env);
      }
      if (target.tag === "member") {
        const object = evalWithAllAny(target.object, env, session);
        if (object.tag === "error") {
          return object;
        }
        const value = evalWithAllAny(expr.value, env, session);
        return value.tag === "error"
          ? value
          : session.memberSet(object.value, target.name, value.value);
      }
      if (target.tag === "index") {
        const object = evalWithAllAny(target.object, env, session);
        if (object.tag === "error") {
          return object;
        }
        const index = evalWithAllAny(target.index, env, session);
        if (index.tag === "error") {
          return index;
        }
        const value = evalWithAllAny(expr.value, env, session);
        return value.tag === "error"
          ? value
          : session.indexSet(object.value, index.value, value.value);
      }
      return fail({ tag: "unknown-syntax", construct: "assignment-target" });
    }
    case "logical": {
      const left = evalWithAllAny(expr.left, env, session);
      if (left.tag === "error") {
        return left;
      }
      if (typeof left.value !== "boolean") {
        return bad(expr.op, "left operand is not a boolean");
      }
      const shortCircuits = expr.op === "&&" ? !left.value : left.value;
      if (shortCircuits) {
        return ok(left.value);
      }
      const right = evalWithAllAny(expr.right, env, session);
      if (right.tag === "error") {
        return right;
      }
      return typeof right.value === "boolean"
        ? ok(right.value)
        : bad(expr.op, "right operand is not a boolean");
    }
    case "conditional": {
      const test = evalWithAllAny(expr.test, env, session);
      if (test.tag === "error") {
        return test;
      }
      return evalWithAllAny(test.value === true ? expr.consequent : expr.alternative, env, session);
    }
    case "binary": {
      const left = evalWithAllAny(expr.left, env, session);
      if (left.tag === "error") {
        return left;
      }
      const right = evalWithAllAny(expr.right, env, session);
      return right.tag === "error" ? right : applyBinaryOperation(expr.op, left.value, right.value);
    }
    case "unary": {
      const operand = evalWithAllAny(expr.operand, env, session);
      return operand.tag === "error" ? operand : applyUnaryOperation(expr.op, operand.value);
    }
    case "arrow": {
      const { params, required, rest } = splitParams(expr.params);
      return ok(makeClosure(params, required, rest, expr.body, env));
    }
    case "member": {
      const object = evalWithAllAny(expr.object, env, session);
      return object.tag === "error" ? object : session.memberGet(object.value, expr.name);
    }
    case "index": {
      const object = evalWithAllAny(expr.object, env, session);
      if (object.tag === "error") {
        return object;
      }
      const index = evalWithAllAny(expr.index, env, session);
      return index.tag === "error" ? index : session.indexGet(object.value, index.value);
    }
    case "number":
    case "string":
    case "boolean":
      return ok(expr.value);
    case "null":
      return ok(null);
    case "undefined":
      return ok(undefined);
    case "variable":
      return session.lookupVariableValue(expr.name, env);
    default:
      return session.evaluate(expr, env);
  }
};

// ---------------------------------------------------------------------
// The derived route: over the conditional expression
// ---------------------------------------------------------------------

/**
 * The `all` form derived over the conditional: `all(a, b, c)` becomes
 * `a ? (b ? c : false) : false`. Each operand appears once, so the
 * derivation evaluates each reached operand once as a conditional test.
 */
export const allToConditional = (operands: ReadonlyArray<Expr>): Expr => {
  const first = operands[0];
  if (first === undefined) {
    return bool(true);
  }
  const second = operands[1];
  return second === undefined
    ? first
    : cond(first, allToConditional(operands.slice(1)), bool(false));
};

/**
 * The `any` form derived over the conditional: `any(a, b, c)` becomes
 * `a ? true : (b ? true : c)`, again one evaluation per reached operand.
 */
export const anyToConditional = (operands: ReadonlyArray<Expr>): Expr => {
  const first = operands[0];
  if (first === undefined) {
    return bool(false);
  }
  const second = operands[1];
  return second === undefined
    ? first
    : cond(first, bool(true), anyToConditional(operands.slice(1)));
};

export function ex_4_04(): string {
  return (
    "The variadic `all` and `any` are special forms beside the binary `logical` node: " +
    "each operand is evaluated at most once, non-boolean operands are a bad-operand " +
    "fault, and evaluation stops at the first decisive operand, so `all(true, true, " +
    "true)` is true, `all(true, false, ...)` is false without touching its tail, and " +
    "`all(1, 2, 3)` faults instead of answering. The extension nodes cannot be embedded " +
    "in shared procedure-body statements; those examples use the derived conditional " +
    "route, which reaches operands once each but silently treats a non-`true` test as " +
    "false instead of faulting — the comparison the exercise asks for."
  );
}
