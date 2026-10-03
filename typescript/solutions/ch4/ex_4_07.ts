// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.7: `let*` as nested grouped bindings. `sequentialToNested`
 * peels off the outermost binding as a one-binding `let` around the
 * rest — `(let* ((v1 e1) (v2 e2) ...) body)` becomes `(let ((v1 e1))
 * (let* ((v2 e2) ...) body))` — and with no bindings left only the body
 * sequence remains, as a zero-binding `let`. The evaluator's case list
 * is exactly the book's closing question answered in the affirmative:
 * one case that rewrites and re-evaluates is enough once `let` is a
 * case of the same evaluator, because the expansion terminates in
 * plain grouped bindings and no non-derived expression is needed.
 */
import { Session } from "../../packages/ch4/src/01-metacircular.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import {
  type CaseClause,
  call,
  type Decl,
  type Expr,
  exprStmt,
  lam,
  type ObjectField,
  param,
  type Stmt,
} from "../../packages/ch4/src/syntax/ast.js";

/** One binding of a sequential binding: a name and its initializer. */
export interface LetStarBinding {
  readonly name: string;
  readonly init: Expr;
}

/** The sequential binding extension node beside the shared syntax. */
export interface LetStarNode {
  readonly tag: "let-star";
  readonly bindings: ReadonlyArray<LetStarBinding>;
  readonly body: ReadonlyArray<Decl | Stmt>;
  readonly span: Expr["span"];
}

/** The grouped binding this file derives from: one name per frame. */
export interface LetNode {
  readonly tag: "let";
  readonly bindings: ReadonlyArray<LetStarBinding>;
  readonly body: ReadonlyArray<Decl | Stmt>;
  readonly span: Expr["span"];
}

/** The syntax this exercise evaluates: shared expressions plus its two forms. */
export type LetStarExpr = Expr | LetNode | LetStarNode;

/** Builds a sequential binding node. */
export const letStarNode = (
  bindings: ReadonlyArray<LetStarBinding>,
  body: ReadonlyArray<Decl | Stmt>,
  span: Expr["span"],
): LetStarNode => ({ tag: "let-star", bindings, body, span });

/** Builds a grouped binding node (the target of the derivation). */
export const letNode = (
  bindings: ReadonlyArray<LetStarBinding>,
  body: ReadonlyArray<Decl | Stmt>,
  span: Expr["span"],
): LetNode => ({ tag: "let", bindings, body, span });

/** The book's derivation of one grouped binding: `((x1, ..., xn) => { body })(e1, ..., en)`. */
export const letToCall = (node: LetNode): Expr =>
  call(
    lam(
      node.bindings.map((binding) => param(binding.name, null, node.span)),
      node.body,
      node.span,
    ),
    node.bindings.map((binding) => binding.init),
    node.span,
  );

/**
 * `let*` to nested grouped bindings: peel the outermost binding into a
 * one-binding `let` around the rest, lowered to a call when it must be
 * placed in the shared statement body; no bindings left, only the body.
 */
export const sequentialToNested = (node: LetStarNode): LetNode => {
  const first = node.bindings[0];
  if (first === undefined) {
    return letNode([], node.body, node.span);
  }
  const rest = node.bindings.slice(1);
  const inner: ReadonlyArray<Decl | Stmt> =
    rest.length === 0
      ? node.body
      : [exprStmt(letToCall(sequentialToNested({ ...node, bindings: rest })), node.span)];
  return letNode([first], inner, node.span);
};

// ---------------------------------------------------------------------
// Lowering let* and grouped bindings at their typed expression boundaries

const lowerExpr = (expr: LetStarExpr): Expr => {
  if (expr.tag === "let-star") {
    const lowered: LetStarNode = {
      ...expr,
      bindings: expr.bindings.map((binding) => ({
        name: binding.name,
        init: lowerExpr(binding.init),
      })),
      body: lowerItems(expr.body),
    };
    return lowerExpr(sequentialToNested(lowered));
  }
  if (expr.tag === "let") {
    return letToCall({
      ...expr,
      bindings: expr.bindings.map((binding) => ({
        name: binding.name,
        init: lowerExpr(binding.init),
      })),
      body: lowerItems(expr.body),
    });
  }
  switch (expr.tag) {
    case "number":
    case "string":
    case "boolean":
    case "null":
    case "undefined":
    case "variable":
      return expr;
    case "template":
      return { ...expr, exprs: expr.exprs.map(lowerExpr) };
    case "array":
      return {
        ...expr,
        elements: expr.elements.map((arg) => ({ kind: arg.kind, expr: lowerExpr(arg.expr) })),
      };
    case "object": {
      const fields: ObjectField[] = expr.fields.map((field) => ({
        key: field.key,
        value: lowerExpr(field.value),
        span: field.span,
      }));
      return { ...expr, fields };
    }
    case "unary":
      return { ...expr, operand: lowerExpr(expr.operand) };
    case "binary":
      return { ...expr, left: lowerExpr(expr.left), right: lowerExpr(expr.right) };
    case "logical":
      return { ...expr, left: lowerExpr(expr.left), right: lowerExpr(expr.right) };
    case "conditional":
      return {
        ...expr,
        test: lowerExpr(expr.test),
        consequent: lowerExpr(expr.consequent),
        alternative: lowerExpr(expr.alternative),
      };
    case "permanent-assign":
    case "assign":
      return { ...expr, target: lowerExpr(expr.target), value: lowerExpr(expr.value) };
    case "if-fail":
      return {
        ...expr,
        expression: lowerExpr(expr.expression),
        fallback: lowerExpr(expr.fallback),
      };
    case "arrow":
      return { ...expr, body: { body: lowerItems(expr.body.body), span: expr.body.span } };
    case "call":
      return {
        ...expr,
        callee: lowerExpr(expr.callee),
        args: expr.args.map((arg) => ({ kind: arg.kind, expr: lowerExpr(arg.expr) })),
      };
    case "member":
      return { ...expr, object: lowerExpr(expr.object) };
    case "index":
      return { ...expr, object: lowerExpr(expr.object), index: lowerExpr(expr.index) };
    case "new-error":
      return { ...expr, args: expr.args.map(lowerExpr) };
    case "new-map":
      return { ...expr, args: expr.args.map(lowerExpr) };
    case "new-set":
      return { ...expr, args: expr.args.map(lowerExpr) };
    case "delay":
      return { ...expr, expr: lowerExpr(expr.expr) };
    case "force":
      return { ...expr, expr: lowerExpr(expr.expr) };
    case "require":
      return { ...expr, condition: lowerExpr(expr.condition) };
    case "choose":
      return { ...expr, alternatives: expr.alternatives.map(lowerExpr) };
    case "ramb":
      return { ...expr, alternatives: expr.alternatives.map(lowerExpr) };
  }
};
function isStmt(item: Decl | Stmt): item is Stmt {
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
}

function lowerStmt(item: Stmt): Stmt {
  const lowered = lowerItem(item);
  if (!isStmt(lowered)) {
    throw new Error("statement lowering produced a declaration");
  }
  return lowered;
}

const lowerItem = (item: Decl | Stmt): Decl | Stmt => {
  switch (item.tag) {
    case "import":
    case "type-decl":
    case "interface-decl":
    case "break":
    case "continue":
      return item;
    case "var-decl":
      return { ...item, init: lowerExpr(item.init) };
    case "function-decl":
      return { ...item, body: { body: lowerItems(item.body.body), span: item.body.span } };
    case "expr-stmt":
      return { ...item, expr: lowerExpr(item.expr) };
    case "return":
      return item.argument === null ? item : { ...item, argument: lowerExpr(item.argument) };
    case "throw":
      return { ...item, argument: lowerExpr(item.argument) };
    case "if": {
      const alternative = item.alternative === null ? null : lowerStmt(item.alternative);
      return {
        ...item,
        test: lowerExpr(item.test),
        consequent: lowerStmt(item.consequent),
        alternative,
      };
    }
    case "while":
      return { ...item, test: lowerExpr(item.test), body: lowerStmt(item.body) };
    case "for-of":
      return { ...item, iterable: lowerExpr(item.iterable), body: lowerStmt(item.body) };
    case "block":
      return { ...item, body: lowerItems(item.body) };
    case "switch": {
      const cases: CaseClause[] = item.cases.map((clause) => ({
        test: lowerExpr(clause.test),
        body: lowerItems(clause.body),
        span: clause.span,
      }));
      return {
        ...item,
        discriminant: lowerExpr(item.discriminant),
        cases,
        defaultBody: item.defaultBody === null ? null : lowerItems(item.defaultBody),
      };
    }
    case "try": {
      const blockOf = (body: {
        readonly body: ReadonlyArray<Decl | Stmt>;
        readonly span: Expr["span"];
      }): {
        readonly body: ReadonlyArray<Decl | Stmt>;
        readonly span: Expr["span"];
      } => ({ body: lowerItems(body.body), span: body.span });
      return {
        ...item,
        block: blockOf(item.block),
        handler:
          item.handler === null
            ? null
            : { param: item.handler.param, body: blockOf(item.handler.body) },
        finalizer: item.finalizer === null ? null : blockOf(item.finalizer),
      };
    }
  }
};

const lowerItems = (items: ReadonlyArray<Decl | Stmt>): ReadonlyArray<Decl | Stmt> =>
  items.map(lowerItem);

/**
 * The evaluator's case for the sequential form: rewrite to nested
 * grouped bindings, lower to the call form, evaluate the result. One
 * case suffices — the book's closing question, answered yes.
 */
export const evalWithLetStar = (
  expr: LetStarExpr,
  env: Env,
  session: Session = new Session("core"),
): Outcome => session.evaluate(lowerExpr(expr), env);

export function ex_4_07(): string {
  return (
    "Sequential binding is nested grouped binding: `sequentialToNested` peels the " +
    "outermost binding into a one-binding let around the rest, and with no bindings left " +
    "only the body sequence remains. The rewrite terminates in plain grouped bindings, so " +
    "the one evaluator case `(evaluate (sequentialToNested exp) env)` the book asks about is " +
    "enough — no non-derived expression is needed. The book's example is 39, each " +
    "initializer sees the previous bindings (7), empty bindings leave the body (42), and a " +
    "sequential binding lowered to a call inside a procedure body answers 8."
  );
}
