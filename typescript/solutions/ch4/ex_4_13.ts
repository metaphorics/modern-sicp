// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.13 (host-language replacement keeping the number and the
 * binding-removal objective): the guest extension `unbind` removes a
 * name's binding where the call is evaluated. The specification this
 * solution pins: `removeBinding` removes the binding from exactly the
 * first frame of the chain that has one — an inner binding can be
 * dropped to expose the outer one while outer frames are never touched —
 * and fails with `unbound-name` when no frame has the name. A later read
 * behaves exactly as if the name had never been bound there: it finds
 * the outer binding, or fails; it is never a TDZ read, because the
 * removed cell is gone rather than uninitialized. Silent success on a
 * missing name would hide real mistakes, so it is an error.
 */
import { extendEnvironment, Session, splitParams } from "../../packages/ch4/src/01-metacircular.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import { findCell, makeCell } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { fail, ok, outcomeOf } from "../../packages/ch4/src/runtime/errors.js";
import {
  isClosure,
  isPrimitive,
  makeArray,
  makeClosure,
  type Value,
} from "../../packages/ch4/src/runtime/value.js";
import { type Decl, type Expr, type Stmt, str } from "../../packages/ch4/src/syntax/ast.js";
import type { Span } from "../../packages/ch4/src/syntax/diagnostics.js";

/** The `unbind` extension node beside the shared syntax. */
export interface UnbindNode {
  readonly tag: "unbind";
  readonly name: Expr;
  readonly span: Span;
}

/** The syntax this exercise evaluates: shared expressions plus unbind. */
export type UnbindExpr = Expr | UnbindNode;

/** Builds an unbind node (the name is an expression, usually a string literal). */
export const unbindNode = (name: Expr, span: Span = name.span): UnbindNode => ({
  tag: "unbind",
  name,
  span,
});

/**
 * Removes the binding for `name` from the first frame of the chain that
 * has one; `unbound-name` when no frame has it.
 */
export const removeBinding = (name: string, env: Env): Outcome => {
  let frame: Env | null = env;
  while (frame !== null) {
    const cell = frame.bindings.get(name);
    if (cell !== undefined) {
      frame.bindings.delete(name);
      return ok(undefined);
    }
    frame = frame.parent;
  }
  return fail({ tag: "unbound-name", name });
};

const applyWithUnbind = (
  procedure: Value,
  args: ReadonlyArray<Value>,
  session: Session,
): Outcome => {
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
    return outcomeOf(execUnbindSequence(procedure.body.body, frame, session));
  }
  if (isPrimitive(procedure)) {
    return procedure.fn(args);
  }
  return fail({ tag: "not-callable", detail: "unbind evaluator: value is not a procedure" });
};

const execUnbindItem = (
  item: Decl | Stmt,
  env: Env,
  session: Session,
): ReturnType<Session["execStatement"]> => {
  switch (item.tag) {
    case "expr-stmt": {
      const value = evalWithUnbind(item.expr, env, session);
      return value.tag === "error"
        ? { tag: "error", error: value.error }
        : { tag: "normal", value: value.value };
    }
    case "return": {
      if (item.argument === null) {
        return { tag: "normal", value: undefined };
      }
      const value = evalWithUnbind(item.argument, env, session);
      return value.tag === "error"
        ? { tag: "error", error: value.error }
        : { tag: "return", value: value.value };
    }
    case "var-decl": {
      const init = evalWithUnbind(item.init, env, session);
      if (init.tag === "error") {
        return { tag: "error", error: init.error };
      }
      env.bindings.set(item.name, makeCell(init.value, true, item.kind === "let"));
      return { tag: "normal", value: init.value };
    }
    case "block": {
      const frame: Env = { bindings: new Map(), parent: env };
      return execUnbindSequence(item.body, frame, session);
    }
    default:
      return session.execItem(item, env);
  }
};

const execUnbindSequence = (
  items: ReadonlyArray<Decl | Stmt>,
  env: Env,
  session: Session,
): ReturnType<Session["execStatement"]> => {
  let last: ReturnType<Session["execStatement"]> = { tag: "normal", value: undefined };
  for (const item of items) {
    last = execUnbindItem(item, env, session);
    if (last.tag !== "normal") {
      return last;
    }
  }
  return last;
};

/**
 * The evaluator's case for the extension: `unbind(name)` removes the
 * binding where the call is evaluated; every other form runs the
 * engine, and procedure bodies recurse through this same dispatch so
 * the extension works inside them.
 */
export const evalWithUnbind = (
  expr: UnbindExpr,
  env: Env,
  session: Session = new Session("core"),
): Outcome => {
  if (expr.tag === "unbind") {
    const name = evalWithUnbind(expr.name, env, session);
    if (name.tag === "error") {
      return name;
    }
    return typeof name.value === "string"
      ? removeBinding(name.value, env)
      : fail({ tag: "bad-operand", operator: "unbind", detail: "name is not a string" });
  }
  if (expr.tag === "call") {
    const procedure = evalWithUnbind(expr.callee, env, session);
    if (procedure.tag === "error") {
      return procedure;
    }
    const args: Value[] = [];
    for (const arg of expr.args) {
      const value = evalWithUnbind(arg.expr, env, session);
      if (value.tag === "error") {
        return value;
      }
      args.push(value.value);
    }
    return applyWithUnbind(procedure.value, args, session);
  }
  return session.evaluate(expr, env);
};

export function ex_4_13(): string {
  return (
    "The extension `unbind` removes the binding from exactly the first frame of the chain " +
    "that has one and fails with unbound-name when no frame has it: an inner binding can " +
    "be dropped to expose the outer one, outer frames are never touched, and a later read " +
    "finds the outer binding or fails — never a TDZ read, because the cell is gone. With " +
    "global a = 1 and an inner a = 2, a plain call answers 2; unbinding first answers 1; " +
    "a retargeting call that unbinds then writes 50 leaves both the call's answer and the " +
    "global a at 50; unbinding zz fails; unbinding the global a makes a later lookup and " +
    "a second unbind both fail."
  );
}
