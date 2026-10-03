// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.9: design iteration constructs as derived forms over the
 * guest's `while` and `for-of`. The two extension nodes lower to core
 * statements: the while expression becomes a loop in an immediately
 * called block, and the range loop binds its variable and its limit
 * once, then walks inclusively with `while`. The loop's value is
 * `undefined`, the statement's own answer; every constructed node
 * carries the source node's span, and the lowering is total so the
 * constructs work inside procedure bodies.
 */
import { Session } from "../../packages/ch4/src/01-metacircular.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import {
  assign,
  bin,
  block,
  type CaseClause,
  call,
  type Decl,
  type Expr,
  exprStmt,
  ident,
  lam,
  type ObjectField,
  type Stmt,
  varDecl,
  whileStmt,
} from "../../packages/ch4/src/syntax/ast.js";
import type { Span } from "../../packages/ch4/src/syntax/diagnostics.js";

/** The while expression extension: loops while its test holds. */
export interface WhileExprNode {
  readonly tag: "while-expr";
  readonly test: Expr;
  readonly body: ReadonlyArray<Decl | Stmt>;
  readonly span: Span;
}

/** The range loop extension: walks `name` from `from` to `to`, inclusive. */
export interface ForRangeNode {
  readonly tag: "for-range";
  readonly name: string;
  readonly from: Expr;
  readonly to: Expr;
  readonly body: ReadonlyArray<Decl | Stmt>;
  readonly span: Span;
}

/** The syntax this exercise evaluates: shared expressions plus its two forms. */
export type IterExpr = Expr | WhileExprNode | ForRangeNode;

/** Builds a while expression. */
export const whileExpr = (
  test: Expr,
  body: ReadonlyArray<Decl | Stmt>,
  span: Span,
): WhileExprNode => ({ tag: "while-expr", test, body, span });

/** Builds a range loop. */
export const forRange = (
  name: string,
  from: Expr,
  to: Expr,
  body: ReadonlyArray<Decl | Stmt>,
  span: Span,
): ForRangeNode => ({ tag: "for-range", name, from, to, body, span });

/** The generated name the range limit is bound to exactly once. */
export const limitName = "@@limit";

/** The while expression derived over the guest's `while` statement. */
export const whileToWhile = (node: WhileExprNode): Expr =>
  call(
    lam(
      [],
      [whileStmt(node.test, { tag: "block", body: node.body, span: node.span }, node.span)],
      node.span,
    ),
    [],
    node.span,
  );

/**
 * The range loop derived over `while`: the variable and the limit are
 * bound once, then the loop walks `name` from `from` to `to` inclusive.
 */
export const forRangeToWhile = (node: ForRangeNode): Expr =>
  call(
    lam(
      [],
      [
        varDecl("let", node.name, node.from, null, node.span),
        varDecl("const", limitName, node.to, null, node.span),
        whileStmt(
          bin("<=", ident(node.name, node.span), ident(limitName, node.span), node.span),
          {
            tag: "block",
            body: [
              ...node.body,
              exprStmt(
                assign(
                  ident(node.name, node.span),
                  bin(
                    "+",
                    ident(node.name, node.span),
                    { tag: "number", value: 1, span: node.span },
                    node.span,
                  ),
                  node.span,
                ),
                node.span,
              ),
            ],
            span: node.span,
          },
          node.span,
        ),
      ],
      node.span,
    ),
    [],
    node.span,
  );

// ---------------------------------------------------------------------
// Total lowering: both extension forms anywhere in the tree
// ---------------------------------------------------------------------

const lowerExpr = (expr: IterExpr): Expr => {
  if (expr.tag === "while-expr") {
    return whileToWhile({ ...expr, test: lowerExpr(expr.test), body: lowerItems(expr.body) });
  }
  if (expr.tag === "for-range") {
    return forRangeToWhile({
      ...expr,
      from: lowerExpr(expr.from),
      to: lowerExpr(expr.to),
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
        readonly span: Span;
      }): {
        readonly body: ReadonlyArray<Decl | Stmt>;
        readonly span: Span;
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

/** The evaluator's case for the derived forms: lower, then evaluate. */
export const evalIteration = (
  expr: IterExpr,
  env: Env,
  session: Session = new Session("core"),
): Outcome => session.evaluate(lowerExpr(expr), env);

export function ex_4_09(): string {
  return (
    "Iteration is a derived form over the guest's `while`: the while expression becomes a " +
    "loop in an immediately called block, and the range loop binds its variable and limit " +
    "once and walks inclusively. A range loop over 1..5 accumulating into `total` answers " +
    "15, matching the manual recursion; a while summing 1..4 answers 10 at top level and " +
    "inside a procedure body; a while with a false predicate never runs and answers " +
    "undefined; a range loop over 3..3 runs its body once and answers 3."
  );
}
