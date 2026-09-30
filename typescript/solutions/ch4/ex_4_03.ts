// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.3: rewrite the dispatch of the evaluator in data-directed
 * style, as in the data-directed differentiation of exercise 2.73. The
 * table is keyed by the node `tag` (the typed syntax carries its own
 * dispatch key; there is no `car`), one operation installed per special
 * form with `put` and fetched with `get`. Self-evaluating literals and
 * variables carry no form, so they bypass the table; a `call` node whose
 * tag is not installed runs the application path; the remaining data tags
 * fall through to the ordinary evaluator's cases. The installed
 * operations take the expression and the environment directly — the
 * book's shape — so one table serves every environment and subexpressions
 * and the expression forms of procedure bodies recurse through the same
 * data-directed dispatch.
 */
import {
  applyBinaryOperation,
  applyUnaryOperation,
  extendEnvironment,
  Session,
  splitParams,
} from "../../packages/ch4/src/01-metacircular.js";
import { type Env, makeCell } from "../../packages/ch4/src/runtime/env.js";
import type { Completion, Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { fail, failed, normal, ok, outcomeOf } from "../../packages/ch4/src/runtime/errors.js";
import {
  isArrayValue,
  isClosure,
  isPrimitive,
  makeArray,
  makeClosure,
  type Value,
} from "../../packages/ch4/src/runtime/value.js";
import type { Decl, Expr, Stmt } from "../../packages/ch4/src/syntax/ast.js";

/** One installed form operation, keyed by the node tag it handles. */
export type FormOperation = (expr: Expr, env: Env, session: Session) => Outcome;

/** The form table of exercise 2.73: one `put` per form, `get` by tag. */
export class FormTable {
  readonly #operations = new Map<Expr["tag"], FormOperation>();

  /** Installs one operation under its node tag; a second put overwrites. */
  put(tag: Expr["tag"], operation: FormOperation): void {
    this.#operations.set(tag, operation);
  }

  /** The operation installed under `tag`, if any. */
  get(tag: Expr["tag"]): FormOperation | undefined {
    return this.#operations.get(tag);
  }
}

const bad = (operator: string, detail: string): Outcome =>
  fail({ tag: "bad-operand", operator, detail });

/** The application path for a `call` node: operator and operands through
 * the data-directed evaluator, then application. */
const applyData = (procedure: Value, args: ReadonlyArray<Value>, session: Session): Outcome => {
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
    return outcomeOf(execDataSequence(procedure.body.body, frame, session));
  }
  if (isPrimitive(procedure)) {
    return procedure.fn(args);
  }
  return fail({ tag: "not-callable", detail: "data-directed eval: value is not a procedure" });
};

/** The form table with one operation installed per special form. */
export const makeFormTable = (): FormTable => {
  const table = new FormTable();

  table.put("assign", (expr, env, session) => {
    if (expr.tag !== "assign") {
      return fail({ tag: "unknown-syntax", construct: "assign" });
    }
    const target = expr.target;
    if (target.tag === "member") {
      const object = evalDataDirected(target.object, env, session);
      if (object.tag === "error") {
        return object;
      }
      const value = evalDataDirected(expr.value, env, session);
      return value.tag === "error"
        ? value
        : session.memberSet(object.value, target.name, value.value);
    }
    if (target.tag === "index") {
      const object = evalDataDirected(target.object, env, session);
      if (object.tag === "error") {
        return object;
      }
      const index = evalDataDirected(target.index, env, session);
      if (index.tag === "error") {
        return index;
      }
      const value = evalDataDirected(expr.value, env, session);
      return value.tag === "error"
        ? value
        : session.indexSet(object.value, index.value, value.value);
    }
    if (target.tag !== "variable") {
      return fail({ tag: "unknown-syntax", construct: "assignment-target" });
    }
    const cell = env.bindings.get(target.name);
    if (cell === undefined) {
      return fail({ tag: "unbound-name", name: target.name });
    }
    const value = evalDataDirected(expr.value, env, session);
    if (value.tag === "error") {
      return value;
    }
    if (!cell.mutable) {
      return bad("=", "assignment to a const binding");
    }
    cell.value = value.value;
    cell.initialized = true;
    return ok(value.value);
  });

  table.put("logical", (expr, env, session) => {
    if (expr.tag !== "logical") {
      return fail({ tag: "unknown-syntax", construct: "logical" });
    }
    const left = evalDataDirected(expr.left, env, session);
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
    const right = evalDataDirected(expr.right, env, session);
    if (right.tag === "error") {
      return right;
    }
    return typeof right.value === "boolean"
      ? ok(right.value)
      : bad(expr.op, "right operand is not a boolean");
  });

  table.put("conditional", (expr, env, session) => {
    if (expr.tag !== "conditional") {
      return fail({ tag: "unknown-syntax", construct: "conditional" });
    }
    const test = evalDataDirected(expr.test, env, session);
    if (test.tag === "error") {
      return test;
    }
    return evalDataDirected(test.value === true ? expr.consequent : expr.alternative, env, session);
  });

  table.put("arrow", (expr, env) => {
    if (expr.tag !== "arrow") {
      return fail({ tag: "unknown-syntax", construct: "arrow" });
    }
    const { params, rest } = splitParams(expr.params);
    return ok(makeClosure(params, rest, expr.body, env));
  });

  table.put("binary", (expr, env, session) => {
    if (expr.tag !== "binary") {
      return fail({ tag: "unknown-syntax", construct: "binary" });
    }
    const left = evalDataDirected(expr.left, env, session);
    if (left.tag === "error") {
      return left;
    }
    const right = evalDataDirected(expr.right, env, session);
    return right.tag === "error" ? right : applyBinaryOperation(expr.op, left.value, right.value);
  });

  table.put("unary", (expr, env, session) => {
    if (expr.tag !== "unary") {
      return fail({ tag: "unknown-syntax", construct: "unary" });
    }
    const operand = evalDataDirected(expr.operand, env, session);
    return operand.tag === "error" ? operand : applyUnaryOperation(expr.op, operand.value);
  });

  table.put("template", (expr, env, session) => {
    if (expr.tag !== "template") {
      return fail({ tag: "unknown-syntax", construct: "template" });
    }
    let text = "";
    for (let i = 0; i < expr.chunks.length; i += 1) {
      text += expr.chunks[i] ?? "";
      const inner = expr.exprs[i];
      if (inner === undefined) {
        continue;
      }
      const value = evalDataDirected(inner, env, session);
      if (value.tag === "error") {
        return value;
      }
      text += session.render(value.value);
    }
    return ok(text);
  });

  return table;
};

/** The shared table; adding a form is one more `put`. */
const standardForms = makeFormTable();

/**
 * The data-directed evaluator: literals and variables bypass the table,
 * an installed tag runs its operation, a `call` runs the application
 * path, and any other uninstalled tag runs the ordinary evaluator.
 */
export const evalDataDirected = (
  expr: Expr,
  env: Env,
  session: Session = new Session("core"),
  table: FormTable = standardForms,
): Outcome => {
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
      return session.lookupVariableValue(expr.name, env);
    default:
      break;
  }
  const operation = table.get(expr.tag);
  if (operation !== undefined) {
    return operation(expr, env, session);
  }
  if (expr.tag === "call") {
    const procedure = evalDataDirected(expr.callee, env, session, table);
    if (procedure.tag === "error") {
      return procedure;
    }
    const args: Value[] = [];
    for (const arg of expr.args) {
      const value = evalDataDirected(arg.expr, env, session, table);
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
    return applyData(procedure.value, args, session);
  }
  return session.evaluate(expr, env);
};

const execDataItem = (
  item: Decl | Stmt,
  env: Env,
  session: Session,
  table: FormTable,
): Completion => {
  switch (item.tag) {
    case "var-decl": {
      const init = evalDataDirected(item.init, env, session, table);
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
      const value = evalDataDirected(item.expr, env, session, table);
      return value.tag === "error" ? failed(value.error) : normal(value.value);
    }
    case "return": {
      if (item.argument === null) {
        return normal(undefined);
      }
      const value = evalDataDirected(item.argument, env, session, table);
      return value.tag === "error" ? failed(value.error) : { tag: "return", value: value.value };
    }
    case "block":
      return execDataSequence(item.body, { bindings: new Map(), parent: env }, session, table);
    case "if": {
      const test = evalDataDirected(item.test, env, session, table);
      if (test.tag === "error") {
        return failed(test.error);
      }
      const branch = test.value === true ? item.consequent : item.alternative;
      return branch === null ? normal(undefined) : execDataItem(branch, env, session, table);
    }
    default:
      return session.execStatement(item, env);
  }
};

/** Runs a procedure body through the same data-directed dispatch. */
export const execDataSequence = (
  items: ReadonlyArray<Decl | Stmt>,
  env: Env,
  session: Session = new Session("core"),
  table: FormTable = standardForms,
): Completion => {
  for (const item of items) {
    if (item.tag === "var-decl") {
      env.bindings.set(item.name, makeCell(undefined, false, item.kind === "let"));
    }
    if (item.tag === "function-decl") {
      const { params, rest } = splitParams(item.params);
      env.bindings.set(item.name, makeCell(makeClosure(params, rest, item.body, env), true));
    }
  }
  let last: Completion = normal(undefined);
  for (const item of items) {
    last = execDataItem(item, env, session, table);
    if (last.tag !== "normal") {
      return last;
    }
  }
  return last;
};

export function ex_4_03(): string {
  return (
    "The dispatch is a table keyed by the node tag, as in the data-directed " +
    "differentiation of 2.73: assignment, logical connectives, conditionals, arrows, " +
    "binary and unary operations, and templates are one operation each, installed with " +
    "put and fetched with get. Literals and variables bypass the table, an uninstalled " +
    "call tag runs the application path, and any other uninstalled tag runs the " +
    "ordinary evaluator. Operations take the expression and environment directly, so " +
    "one table serves every environment and bodies recurse through the same dispatch; " +
    "adding a form is one more put, not one more case."
  );
}
