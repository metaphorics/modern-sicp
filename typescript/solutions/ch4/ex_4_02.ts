// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.2: (a) Louis Reasoner moves the procedure-application case
 * in front of the assignment case. Over the book's pair data that breaks:
 * a definition is a pair, so it reaches the application case and the
 * evaluator tries to apply the unbound name `define`. The shared typed
 * syntax cannot reproduce the collision — `Expr` is a tagged union with
 * disjoint cases, and `const x = 3;` is a `var-decl` declaration, not an
 * expression at all — so `evalApplicationsFirst` re-assembles the whole
 * case analysis with the `call` case first and is observationally equal
 * to `evaluate`. (b) The call-tagged spelling `call(f, a, b)` is a
 * reader-level dialect over the same syntax: `readCallDialect` lowers it
 * to the ordinary `call` node, so the evaluator is unchanged, and a bare
 * `f(a, b)` application is rejected at read time as `unknown-syntax`.
 */
import {
  applyBinaryOperation,
  applyUnaryOperation,
  Session,
  setVariableValue,
  splitParams,
} from "../../packages/ch4/src/01-metacircular.js";
import { read } from "../../packages/ch4/src/read.js";
import { type Env, findCell } from "../../packages/ch4/src/runtime/env.js";
import type { GuestError, Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { fail, ok } from "../../packages/ch4/src/runtime/errors.js";
import {
  isArrayValue,
  isThunkValue,
  makeArray,
  makeClosure,
  makeErrorValue,
  makeMap,
  makeRecord,
  makeSet,
  ThunkValue,
  type Value,
} from "../../packages/ch4/src/runtime/value.js";
import type {
  Arg,
  Block,
  CaseClause,
  Decl,
  Expr,
  ObjectField,
  Stmt,
} from "../../packages/ch4/src/syntax/ast.js";

/** Evaluated arguments or the error that stopped them. */
type Args =
  | { readonly tag: "args"; readonly values: Value[] }
  | { readonly tag: "error"; readonly error: GuestError };

/** A lowered syntax node or the read-time rejection that stopped it. */
type Lowered<A> =
  | { readonly tag: "ok"; readonly node: A }
  | { readonly tag: "error"; readonly error: GuestError };

const bad = (operator: string, detail: string): Outcome =>
  fail({ tag: "bad-operand", operator, detail });

// ---------------------------------------------------------------------
// (a) Louis's dispatch order over the typed syntax
// ---------------------------------------------------------------------

const evalArgsLr = (args: ReadonlyArray<Arg>, env: Env, session: Session): Args => {
  const values: Value[] = [];
  for (const arg of args) {
    const value = evalApplicationsFirst(arg.expr, env, session);
    if (value.tag === "error") {
      return value;
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
};

const evalAssignmentFirst = (
  expr: Extract<Expr, { tag: "assign" }>,
  env: Env,
  session: Session,
): Outcome => {
  const target = expr.target;
  if (target.tag === "variable") {
    const cell = findCell(env, target.name);
    if (cell === undefined) {
      return fail({ tag: "unbound-name", name: target.name });
    }
    const value = evalApplicationsFirst(expr.value, env, session);
    if (value.tag === "error") {
      return value;
    }
    return setVariableValue(target.name, value.value, env);
  }
  if (target.tag === "member") {
    const object = evalApplicationsFirst(target.object, env, session);
    if (object.tag === "error") {
      return object;
    }
    const value = evalApplicationsFirst(expr.value, env, session);
    return value.tag === "error"
      ? value
      : session.memberSet(object.value, target.name, value.value);
  }
  if (target.tag === "index") {
    const object = evalApplicationsFirst(target.object, env, session);
    if (object.tag === "error") {
      return object;
    }
    const index = evalApplicationsFirst(target.index, env, session);
    if (index.tag === "error") {
      return index;
    }
    const value = evalApplicationsFirst(expr.value, env, session);
    return value.tag === "error" ? value : session.indexSet(object.value, index.value, value.value);
  }
  return fail({ tag: "unknown-syntax", construct: "assignment-target" });
};

const evalNewMapFirst = (args: ReadonlyArray<Expr>, env: Env, session: Session): Outcome => {
  const first = args[0];
  if (first === undefined) {
    return ok(makeMap());
  }
  const entriesValue = evalApplicationsFirst(first, env, session);
  if (entriesValue.tag === "error") {
    return entriesValue;
  }
  if (!isArrayValue(entriesValue.value)) {
    return bad("new Map", "argument is not an entries array");
  }
  const entries: Array<readonly [Value, Value]> = [];
  for (const pair of entriesValue.value.items) {
    if (!isArrayValue(pair) || pair.items.length !== 2) {
      return bad("new Map", "entry is not a two-element array");
    }
    entries.push([pair.items[0], pair.items[1]]);
  }
  return ok(makeMap(entries));
};

/**
 * Louis's order: the application case before assignment. Every case is
 * re-assembled here and subexpressions recurse through this same case
 * analysis, so the only difference from `evaluate` is the order of the
 * cases — which disjoint tags make unobservable.
 */
export const evalApplicationsFirst = (
  expr: Expr,
  env: Env,
  session: Session = new Session("core"),
): Outcome => {
  switch (expr.tag) {
    case "call": {
      const procedure = evalApplicationsFirst(expr.callee, env, session);
      if (procedure.tag === "error") {
        return procedure;
      }
      const args = evalArgsLr(expr.args, env, session);
      if (args.tag === "error") {
        return fail(args.error);
      }
      return session.applyProcedure(procedure.value, args.values);
    }
    case "assign":
      return evalAssignmentFirst(expr, env, session);
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
    case "template": {
      let text = "";
      for (let i = 0; i < expr.chunks.length; i += 1) {
        text += expr.chunks[i] ?? "";
        const inner = expr.exprs[i];
        if (inner === undefined) {
          continue;
        }
        const value = evalApplicationsFirst(inner, env, session);
        if (value.tag === "error") {
          return value;
        }
        text += session.render(value.value);
      }
      return ok(text);
    }
    case "array": {
      const args = evalArgsLr(expr.elements, env, session);
      return args.tag === "error" ? fail(args.error) : ok(makeArray(args.values));
    }
    case "object": {
      const entries: Array<readonly [string, Value]> = [];
      for (const field of expr.fields) {
        const value = evalApplicationsFirst(field.value, env, session);
        if (value.tag === "error") {
          return value;
        }
        entries.push([field.key, value.value]);
      }
      return ok(makeRecord(entries));
    }
    case "unary": {
      const operand = evalApplicationsFirst(expr.operand, env, session);
      return operand.tag === "error" ? operand : applyUnaryOperation(expr.op, operand.value);
    }
    case "binary": {
      const left = evalApplicationsFirst(expr.left, env, session);
      if (left.tag === "error") {
        return left;
      }
      const right = evalApplicationsFirst(expr.right, env, session);
      return right.tag === "error" ? right : applyBinaryOperation(expr.op, left.value, right.value);
    }
    case "logical": {
      const left = evalApplicationsFirst(expr.left, env, session);
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
      const right = evalApplicationsFirst(expr.right, env, session);
      if (right.tag === "error") {
        return right;
      }
      return typeof right.value === "boolean"
        ? ok(right.value)
        : bad(expr.op, "right operand is not a boolean");
    }
    case "conditional": {
      const test = evalApplicationsFirst(expr.test, env, session);
      if (test.tag === "error") {
        return test;
      }
      return evalApplicationsFirst(
        test.value === true ? expr.consequent : expr.alternative,
        env,
        session,
      );
    }
    case "arrow": {
      const { params, required, rest } = splitParams(expr.params);
      return ok(makeClosure(params, required, rest, expr.body, env));
    }
    case "member": {
      const object = evalApplicationsFirst(expr.object, env, session);
      return object.tag === "error" ? object : session.memberGet(object.value, expr.name);
    }
    case "index": {
      const object = evalApplicationsFirst(expr.object, env, session);
      if (object.tag === "error") {
        return object;
      }
      const index = evalApplicationsFirst(expr.index, env, session);
      return index.tag === "error" ? index : session.indexGet(object.value, index.value);
    }
    case "new-error": {
      const first = expr.args[0];
      if (first === undefined) {
        return ok(makeErrorValue(""));
      }
      const message = evalApplicationsFirst(first, env, session);
      return message.tag === "error" ? message : ok(makeErrorValue(session.render(message.value)));
    }
    case "new-map":
      return evalNewMapFirst(expr.args, env, session);
    case "new-set": {
      const first = expr.args[0];
      if (first === undefined) {
        return ok(makeSet());
      }
      const items = evalApplicationsFirst(first, env, session);
      if (items.tag === "error") {
        return items;
      }
      return isArrayValue(items.value)
        ? ok(makeSet(items.value.items))
        : bad("new Set", "argument is not an array");
    }
    case "delay":
      return ok(new ThunkValue(expr.expr, env));
    case "force": {
      const thunkValue = evalApplicationsFirst(expr.expr, env, session);
      if (thunkValue.tag === "error") {
        return thunkValue;
      }
      if (!isThunkValue(thunkValue.value)) {
        return bad("force", "argument is not a thunk");
      }
      return evalApplicationsFirst(thunkValue.value.expr, thunkValue.value.env, session);
    }
    case "choose":
    case "ramb":
    case "require":
    case "permanent-assign":
    case "if-fail":
      return fail({
        tag: "unknown-syntax",
        construct: "search-experiment (run through runAmbSource)",
      });
  }
};

// ---------------------------------------------------------------------
// (b) The call-tagged reader-level dialect
// ---------------------------------------------------------------------

const bareApplication = (): Lowered<never> => ({
  tag: "error",
  error: {
    tag: "unknown-syntax",
    construct: "application in the call-tagged dialect (write call(f, a, b))",
  },
});

const lowerExprs = (exprs: ReadonlyArray<Expr>): Lowered<ReadonlyArray<Expr>> => {
  const out: Expr[] = [];
  for (const expr of exprs) {
    const lowered = lowerExpr(expr);
    if (lowered.tag === "error") {
      return lowered;
    }
    out.push(lowered.node);
  }
  return { tag: "ok", node: out };
};

const lowerArgs = (args: ReadonlyArray<Arg>): Lowered<ReadonlyArray<Arg>> => {
  const out: Arg[] = [];
  for (const arg of args) {
    const lowered = lowerExpr(arg.expr);
    if (lowered.tag === "error") {
      return lowered;
    }
    out.push({ kind: arg.kind, expr: lowered.node });
  }
  return { tag: "ok", node: out };
};

const lowerItems = (items: ReadonlyArray<Decl | Stmt>): Lowered<ReadonlyArray<Decl | Stmt>> => {
  const out: Array<Decl | Stmt> = [];
  for (const item of items) {
    const lowered = lowerItem(item);
    if (lowered.tag === "error") {
      return lowered;
    }
    out.push(lowered.node);
  }
  return { tag: "ok", node: out };
};

const lowerBlock = (body: Block): Lowered<Block> => {
  const items = lowerItems(body.body);
  return items.tag === "error" ? items : { tag: "ok", node: { body: items.node, span: body.span } };
};
const isStmt = (item: Decl | Stmt): item is Stmt => {
  switch (item.tag) {
    case "import":
    case "type-decl":
    case "interface-decl":
    case "var-decl":
    case "function-decl":
      return false;
    default:
      return true;
  }
};

const lowerItem = (item: Decl | Stmt): Lowered<Decl | Stmt> => {
  switch (item.tag) {
    case "import":
    case "type-decl":
    case "interface-decl":
    case "break":
    case "continue":
      return { tag: "ok", node: item };
    case "var-decl": {
      const init = lowerExpr(item.init);
      return init.tag === "error" ? init : { tag: "ok", node: { ...item, init: init.node } };
    }
    case "function-decl": {
      const body = lowerBlock(item.body);
      return body.tag === "error" ? body : { tag: "ok", node: { ...item, body: body.node } };
    }
    case "expr-stmt": {
      const expr = lowerExpr(item.expr);
      return expr.tag === "error" ? expr : { tag: "ok", node: { ...item, expr: expr.node } };
    }
    case "return": {
      if (item.argument === null) {
        return { tag: "ok", node: item };
      }
      const argument = lowerExpr(item.argument);
      return argument.tag === "error"
        ? argument
        : { tag: "ok", node: { ...item, argument: argument.node } };
    }
    case "throw": {
      const argument = lowerExpr(item.argument);
      return argument.tag === "error"
        ? argument
        : { tag: "ok", node: { ...item, argument: argument.node } };
    }
    case "if": {
      const test = lowerExpr(item.test);
      if (test.tag === "error") {
        return test;
      }
      const consequent = lowerItem(item.consequent);
      if (consequent.tag === "error") {
        return consequent;
      }
      if (!isStmt(consequent.node)) {
        return {
          tag: "error",
          error: { tag: "unknown-syntax", construct: "declaration lowered as if-branch" },
        };
      }
      if (item.alternative === null) {
        return {
          tag: "ok",
          node: { ...item, test: test.node, consequent: consequent.node, alternative: null },
        };
      }
      const alternative = lowerItem(item.alternative);
      if (alternative.tag === "error") {
        return alternative;
      }
      if (!isStmt(alternative.node)) {
        return {
          tag: "error",
          error: { tag: "unknown-syntax", construct: "declaration lowered as if-branch" },
        };
      }
      return {
        tag: "ok",
        node: {
          ...item,
          test: test.node,
          consequent: consequent.node,
          alternative: alternative.node,
        },
      };
    }
    case "while": {
      const test = lowerExpr(item.test);
      if (test.tag === "error") {
        return test;
      }
      const body = lowerItem(item.body);
      if (body.tag === "error") {
        return body;
      }
      if (!isStmt(body.node)) {
        return {
          tag: "error",
          error: { tag: "unknown-syntax", construct: "declaration lowered as while body" },
        };
      }
      return { tag: "ok", node: { ...item, test: test.node, body: body.node } };
    }
    case "for-of": {
      const iterable = lowerExpr(item.iterable);
      if (iterable.tag === "error") {
        return iterable;
      }
      const body = lowerItem(item.body);
      if (body.tag === "error") {
        return body;
      }
      if (!isStmt(body.node)) {
        return {
          tag: "error",
          error: { tag: "unknown-syntax", construct: "declaration lowered as for-of body" },
        };
      }
      return { tag: "ok", node: { ...item, iterable: iterable.node, body: body.node } };
    }
    case "block": {
      const body = lowerItems(item.body);
      return body.tag === "error" ? body : { tag: "ok", node: { ...item, body: body.node } };
    }
    case "switch": {
      const discriminant = lowerExpr(item.discriminant);
      if (discriminant.tag === "error") {
        return discriminant;
      }
      const cases: CaseClause[] = [];
      for (const clause of item.cases) {
        const test = lowerExpr(clause.test);
        if (test.tag === "error") {
          return test;
        }
        const body = lowerItems(clause.body);
        if (body.tag === "error") {
          return body;
        }
        cases.push({ test: test.node, body: body.node, span: clause.span });
      }
      if (item.defaultBody === null) {
        return {
          tag: "ok",
          node: { ...item, discriminant: discriminant.node, cases, defaultBody: null },
        };
      }
      const defaultBody = lowerItems(item.defaultBody);
      return defaultBody.tag === "error"
        ? defaultBody
        : {
            tag: "ok",
            node: {
              ...item,
              discriminant: discriminant.node,
              cases,
              defaultBody: defaultBody.node,
            },
          };
    }
    case "try": {
      const block = lowerBlock(item.block);
      if (block.tag === "error") {
        return block;
      }
      let handler: { readonly param: string | null; readonly body: Block } | null = null;
      if (item.handler !== null) {
        const handlerBody = lowerBlock(item.handler.body);
        if (handlerBody.tag === "error") {
          return handlerBody;
        }
        handler = { param: item.handler.param, body: handlerBody.node };
      }
      if (item.finalizer === null) {
        return {
          tag: "ok",
          node: {
            ...item,
            block: block.node,
            handler,
            finalizer: null,
          },
        };
      }
      const finalizer = lowerBlock(item.finalizer);
      return finalizer.tag === "error"
        ? finalizer
        : {
            tag: "ok",
            node: {
              ...item,
              block: block.node,
              handler,
              finalizer: finalizer.node,
            },
          };
    }
  }
};

// ---------------------------------------------------------------------
// (b) The call-tagged reader-level dialect
// ---------------------------------------------------------------------

const lowerExpr = (expr: Expr): Lowered<Expr> => {
  switch (expr.tag) {
    case "number":
    case "string":
    case "boolean":
    case "null":
    case "undefined":
    case "variable":
      return { tag: "ok", node: expr };
    case "template": {
      const exprs = lowerExprs(expr.exprs);
      return exprs.tag === "error" ? exprs : { tag: "ok", node: { ...expr, exprs: exprs.node } };
    }
    case "array": {
      const elements = lowerArgs(expr.elements);
      return elements.tag === "error"
        ? elements
        : { tag: "ok", node: { ...expr, elements: elements.node } };
    }
    case "object": {
      const fields: ObjectField[] = [];
      for (const field of expr.fields) {
        const value = lowerExpr(field.value);
        if (value.tag === "error") {
          return value;
        }
        fields.push({ key: field.key, value: value.node, span: field.span });
      }
      return { tag: "ok", node: { ...expr, fields } };
    }
    case "unary": {
      const operand = lowerExpr(expr.operand);
      return operand.tag === "error"
        ? operand
        : { tag: "ok", node: { ...expr, operand: operand.node } };
    }
    case "binary": {
      const left = lowerExpr(expr.left);
      if (left.tag === "error") {
        return left;
      }
      const right = lowerExpr(expr.right);
      return right.tag === "error"
        ? right
        : { tag: "ok", node: { ...expr, left: left.node, right: right.node } };
    }
    case "logical": {
      const left = lowerExpr(expr.left);
      if (left.tag === "error") {
        return left;
      }
      const right = lowerExpr(expr.right);
      return right.tag === "error"
        ? right
        : { tag: "ok", node: { ...expr, left: left.node, right: right.node } };
    }
    case "conditional": {
      const test = lowerExpr(expr.test);
      if (test.tag === "error") {
        return test;
      }
      const consequent = lowerExpr(expr.consequent);
      if (consequent.tag === "error") {
        return consequent;
      }
      const alternative = lowerExpr(expr.alternative);
      return alternative.tag === "error"
        ? alternative
        : {
            tag: "ok",
            node: {
              ...expr,
              test: test.node,
              consequent: consequent.node,
              alternative: alternative.node,
            },
          };
    }
    case "permanent-assign":
    case "assign": {
      const target = lowerExpr(expr.target);
      if (target.tag === "error") {
        return target;
      }
      const value = lowerExpr(expr.value);
      return value.tag === "error"
        ? value
        : { tag: "ok", node: { ...expr, target: target.node, value: value.node } };
    }
    case "if-fail": {
      const expression = lowerExpr(expr.expression);
      if (expression.tag === "error") {
        return expression;
      }
      const fallback = lowerExpr(expr.fallback);
      return fallback.tag === "error"
        ? fallback
        : { tag: "ok", node: { ...expr, expression: expression.node, fallback: fallback.node } };
    }
    case "arrow": {
      const body = lowerBlock(expr.body);
      return body.tag === "error" ? body : { tag: "ok", node: { ...expr, body: body.node } };
    }
    case "call": {
      const lowered = lowerArgs(expr.args);
      if (lowered.tag === "error") {
        return lowered;
      }
      const args = lowered.node;
      const operator = args[0];
      const spelledCall = expr.callee.tag === "variable" && expr.callee.name === "call";
      if (!spelledCall || operator === undefined || operator.kind !== "item") {
        return bareApplication();
      }
      return { tag: "ok", node: { ...expr, callee: operator.expr, args: args.slice(1) } };
    }
    case "member": {
      const object = lowerExpr(expr.object);
      return object.tag === "error"
        ? object
        : { tag: "ok", node: { ...expr, object: object.node } };
    }
    case "index": {
      const object = lowerExpr(expr.object);
      if (object.tag === "error") {
        return object;
      }
      const index = lowerExpr(expr.index);
      return index.tag === "error"
        ? index
        : { tag: "ok", node: { ...expr, object: object.node, index: index.node } };
    }
    case "new-error": {
      const args = lowerExprs(expr.args);
      return args.tag === "error" ? args : { tag: "ok", node: { ...expr, args: args.node } };
    }
    case "new-map": {
      const args = lowerExprs(expr.args);
      return args.tag === "error" ? args : { tag: "ok", node: { ...expr, args: args.node } };
    }
    case "new-set": {
      const args = lowerExprs(expr.args);
      return args.tag === "error" ? args : { tag: "ok", node: { ...expr, args: args.node } };
    }
    case "delay": {
      const inner = lowerExpr(expr.expr);
      return inner.tag === "error" ? inner : { tag: "ok", node: { ...expr, expr: inner.node } };
    }
    case "force": {
      const inner = lowerExpr(expr.expr);
      return inner.tag === "error" ? inner : { tag: "ok", node: { ...expr, expr: inner.node } };
    }
    case "require": {
      const condition = lowerExpr(expr.condition);
      return condition.tag === "error"
        ? condition
        : { tag: "ok", node: { ...expr, condition: condition.node } };
    }
    case "choose": {
      const alternatives = lowerExprs(expr.alternatives);
      return alternatives.tag === "error"
        ? alternatives
        : { tag: "ok", node: { ...expr, alternatives: alternatives.node } };
    }
    case "ramb": {
      const alternatives = lowerExprs(expr.alternatives);
      return alternatives.tag === "error"
        ? alternatives
        : { tag: "ok", node: { ...expr, alternatives: alternatives.node } };
    }
  }
};

/** Lowers one call-tagged form to the shared syntax. */
export const lowerCallDialect = (expr: Expr): Lowered<Expr> => lowerExpr(expr);

/** The call-tagged reader: parse the form, then lower it. A bare
 * application is rejected here, at read time, before any evaluation. */
export const readCallDialect = (text: string): Lowered<Expr> => lowerExpr(read(text));

/** Runs one call-tagged source form on the unchanged evaluator. */
export const evalCallTagged = (
  text: string,
  env: Env,
  session: Session = new Session("core"),
): Outcome => {
  const lowered = readCallDialect(text);
  return lowered.tag === "error" ? fail(lowered.error) : session.evaluate(lowered.node, env);
};

export function ex_4_02(): string {
  return (
    "(a) Moving the application case in front of assignment breaks only on pair data, " +
    "where a definition is itself a pair; the tagged syntax has disjoint cases and a " +
    "declaration is not an expression, so `const x = 3;` still binds and the reordered " +
    "case analysis answers every input exactly as `evaluate` does. (b) The call-tagged " +
    "spelling is a reader-level dialect: `call(f, a, b)` lowers to the ordinary call " +
    "node, so the evaluator is unchanged, `call(add, 1, 2)` and `add(1, 2)` both answer " +
    "3, and a bare application in the dialect is rejected at read time as unknown-syntax."
  );
}
