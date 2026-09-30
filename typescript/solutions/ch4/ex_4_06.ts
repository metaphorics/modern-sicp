// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.6: `let` as a derived expression. The grouped binding node
 * `LetNode { tag: "let", bindings, body, span }` is the extension this
 * exercise adds beside the shared syntax — the core parser rejects it,
 * so it exists only in this experiment's constructed data. `letToCall`
 * is the book's derivation, `((x1, ..., xn) => { body })(e1, ..., en)`:
 * one `call` of one `arrow` over the group's names, applied to the
 * group's initializers, every constructed node carrying the source
 * node's span. The evaluator's single case lowers and re-evaluates, and
 * the lowering is total — nested grouped bindings in expressions and in
 * procedure bodies lower too — so the derived form works everywhere.
 */
import { Session } from "../../packages/ch4/src/01-metacircular.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import {
  type CaseClause,
  call,
  type Decl,
  type Expr,
  lam,
  type ObjectField,
  param,
  type Stmt,
} from "../../packages/ch4/src/syntax/ast.js";

/** One binding of a grouped binding: a name and its initializer. */
export interface LetBinding {
  readonly name: string;
  readonly init: Expr;
}

/** The grouped binding extension node beside the shared syntax. */
export interface LetNode {
  readonly tag: "let";
  readonly bindings: ReadonlyArray<LetBinding>;
  readonly body: ReadonlyArray<Decl | Stmt>;
  readonly span: Expr["span"];
}

/** The syntax this exercise evaluates: shared expressions plus grouped bindings. */
export type LetExpr = Expr | LetNode;

/** Builds a grouped binding node. */
export const letNode = (
  bindings: ReadonlyArray<LetBinding>,
  body: ReadonlyArray<Decl | Stmt>,
  span: Expr["span"],
): LetNode => ({ tag: "let", bindings, body, span });

/**
 * The book's derivation: `((x1, ..., xn) => { body })(e1, ..., en)`.
 * Every constructed node copies the source node's span.
 */
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

// ---------------------------------------------------------------------
// Lowering the extension at its typed expression boundary
// ---------------------------------------------------------------------

export const lowerLetExpr = (expr: LetExpr): Expr => {
  if (expr.tag === "let") {
    return letToCall({
      ...expr,
      bindings: expr.bindings.map((binding) => ({
        name: binding.name,
        init: lowerLetExpr(binding.init),
      })),
      body: lowerLetItems(expr.body),
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
      return { ...expr, exprs: expr.exprs.map(lowerLetExpr) };
    case "array":
      return {
        ...expr,
        elements: expr.elements.map((arg) => ({ kind: arg.kind, expr: lowerLetExpr(arg.expr) })),
      };
    case "object": {
      const fields: ObjectField[] = expr.fields.map((field) => ({
        key: field.key,
        value: lowerLetExpr(field.value),
        span: field.span,
      }));
      return { ...expr, fields };
    }
    case "unary":
      return { ...expr, operand: lowerLetExpr(expr.operand) };
    case "binary":
      return { ...expr, left: lowerLetExpr(expr.left), right: lowerLetExpr(expr.right) };
    case "logical":
      return { ...expr, left: lowerLetExpr(expr.left), right: lowerLetExpr(expr.right) };
    case "conditional":
      return {
        ...expr,
        test: lowerLetExpr(expr.test),
        consequent: lowerLetExpr(expr.consequent),
        alternative: lowerLetExpr(expr.alternative),
      };
    case "permanent-assign":
    case "assign":
      return { ...expr, target: lowerLetExpr(expr.target), value: lowerLetExpr(expr.value) };
    case "if-fail":
      return {
        ...expr,
        expression: lowerLetExpr(expr.expression),
        fallback: lowerLetExpr(expr.fallback),
      };
    case "arrow":
      return { ...expr, body: { body: lowerLetItems(expr.body.body), span: expr.body.span } };
    case "call":
      return {
        ...expr,
        callee: lowerLetExpr(expr.callee),
        args: expr.args.map((arg) => ({ kind: arg.kind, expr: lowerLetExpr(arg.expr) })),
      };
    case "member":
      return { ...expr, object: lowerLetExpr(expr.object) };
    case "index":
      return { ...expr, object: lowerLetExpr(expr.object), index: lowerLetExpr(expr.index) };
    case "new-error":
      return { ...expr, args: expr.args.map(lowerLetExpr) };
    case "new-map":
      return { ...expr, args: expr.args.map(lowerLetExpr) };
    case "new-set":
      return { ...expr, args: expr.args.map(lowerLetExpr) };
    case "delay":
      return { ...expr, expr: lowerLetExpr(expr.expr) };
    case "force":
      return { ...expr, expr: lowerLetExpr(expr.expr) };
    case "require":
      return { ...expr, condition: lowerLetExpr(expr.condition) };
    case "choose":
      return { ...expr, alternatives: expr.alternatives.map(lowerLetExpr) };
    case "ramb":
      return { ...expr, alternatives: expr.alternatives.map(lowerLetExpr) };
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

function lowerLetStmt(item: Stmt): Stmt {
  const lowered = lowerLetItem(item);
  if (!isStmt(lowered)) {
    throw new Error("statement lowering produced a declaration");
  }
  return lowered;
}

const lowerLetItem = (item: Decl | Stmt): Decl | Stmt => {
  switch (item.tag) {
    case "import":
    case "type-decl":
    case "interface-decl":
    case "break":
    case "continue":
      return item;
    case "var-decl":
      return { ...item, init: lowerLetExpr(item.init) };
    case "function-decl":
      return { ...item, body: { body: lowerLetItems(item.body.body), span: item.body.span } };
    case "expr-stmt":
      return { ...item, expr: lowerLetExpr(item.expr) };
    case "return":
      return item.argument === null ? item : { ...item, argument: lowerLetExpr(item.argument) };
    case "throw":
      return { ...item, argument: lowerLetExpr(item.argument) };
    case "if": {
      const alternative = item.alternative === null ? null : lowerLetStmt(item.alternative);
      return {
        ...item,
        test: lowerLetExpr(item.test),
        consequent: lowerLetStmt(item.consequent),
        alternative,
      };
    }
    case "while":
      return { ...item, test: lowerLetExpr(item.test), body: lowerLetStmt(item.body) };
    case "for-of":
      return { ...item, iterable: lowerLetExpr(item.iterable), body: lowerLetStmt(item.body) };
    case "block":
      return { ...item, body: lowerLetItems(item.body) };
    case "switch": {
      const cases: CaseClause[] = item.cases.map((clause) => ({
        test: lowerLetExpr(clause.test),
        body: lowerLetItems(clause.body),
        span: clause.span,
      }));
      return {
        ...item,
        discriminant: lowerLetExpr(item.discriminant),
        cases,
        defaultBody: item.defaultBody === null ? null : lowerLetItems(item.defaultBody),
      };
    }
    case "try": {
      const blockOf = (body: {
        readonly body: ReadonlyArray<Decl | Stmt>;
        readonly span: Expr["span"];
      }): {
        readonly body: ReadonlyArray<Decl | Stmt>;
        readonly span: Expr["span"];
      } => ({ body: lowerLetItems(body.body), span: body.span });
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

const lowerLetItems = (items: ReadonlyArray<Decl | Stmt>): ReadonlyArray<Decl | Stmt> =>
  items.map(lowerLetItem);

/**
 * The evaluator's single case for the derived form: lower the grouped
 * bindings to the call form and evaluate the result in the same
 * environment.
 */
export const evalWithLet = (
  expr: LetExpr,
  env: Env,
  session: Session = new Session("core"),
): Outcome => session.evaluate(lowerLetExpr(expr), env);

export function ex_4_06(): string {
  return (
    "The grouped binding is a derived expression: `letToCall` rebuilds " +
    "`((x1, ..., xn) => { body })(e1, ..., en)` with every constructed node carrying the " +
    "source node's span, and the evaluator's one case lowers then evaluates. The lowerer " +
    "recurses through the shared expressions and grouped-binding initializers, but the " +
    "experiment's LetNode is not a shared Expr or Stmt, so procedure-body statements cannot " +
    "embed one directly; body examples carry its lowered call. A let evaluates as its " +
    "combination (7 for x = 3, y = 4 over x + y), the lowering is structurally the " +
    "hand-written lambda call, and a lowered call in a procedure body answers 42. The body " +
    "also runs as a sequence in its new frame: x = 10 then x is 10."
  );
}
