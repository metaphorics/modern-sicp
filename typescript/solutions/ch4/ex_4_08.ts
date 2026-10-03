// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.8: named `let`. The named binding is a local named function
 * over the group's names, called on the initializers: `let loop((v1 e1),
 * ...) body` becomes `(() => { function loop(v1, ...) { body } return
 * loop(e1, ...); })()`. The function declaration binds the loop name in
 * the local frame before the body can call it, so recursive calls in the
 * body resolve to the loop itself; the call at the end starts it with
 * the initializers. Plain grouped bindings stay in the same evaluator
 * through exercise 4.6's derivation, so one lowering handles both.
 */
import { Session } from "../../packages/ch4/src/01-metacircular.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import {
  type CaseClause,
  call,
  type Decl,
  type Expr,
  functionDecl,
  lam,
  type ObjectField,
  param,
  returnStmt,
  type Stmt,
} from "../../packages/ch4/src/syntax/ast.js";

/** One binding of a named or grouped binding: a name and its initializer. */
export interface LoopBinding {
  readonly name: string;
  readonly init: Expr;
}

/** The named binding extension node beside the shared syntax. */
export interface NamedLetNode {
  readonly tag: "named-let";
  readonly name: string;
  readonly bindings: ReadonlyArray<LoopBinding>;
  readonly body: ReadonlyArray<Decl | Stmt>;
  readonly span: Expr["span"];
}

/** The grouped binding kept alongside for plain lets. */
export interface LetNode {
  readonly tag: "let";
  readonly bindings: ReadonlyArray<LoopBinding>;
  readonly body: ReadonlyArray<Decl | Stmt>;
  readonly span: Expr["span"];
}

/** The syntax this exercise evaluates: shared expressions plus its two forms. */
export type NamedLetExpr = Expr | LetNode | NamedLetNode;

/** Builds a named binding node. */
export const namedLetNode = (
  name: string,
  bindings: ReadonlyArray<LoopBinding>,
  body: ReadonlyArray<Decl | Stmt>,
  span: Expr["span"],
): NamedLetNode => ({ tag: "named-let", name, bindings, body, span });

/** Builds a grouped binding node. */
export const letNode = (
  bindings: ReadonlyArray<LoopBinding>,
  body: ReadonlyArray<Decl | Stmt>,
  span: Expr["span"],
): LetNode => ({ tag: "let", bindings, body, span });

/** Plain grouped bindings derive to the call form (exercise 4.6). */
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
 * The named form derives to a local named function over the group's
 * names, called on the initializers.
 */
export const namedLetToCall = (node: NamedLetNode): Expr =>
  call(
    lam(
      [],
      [
        functionDecl(
          node.name,
          node.bindings.map((binding) => param(binding.name, null, node.span)),
          node.body,
          null,
          node.span,
        ),
        returnStmt(
          call(
            { tag: "variable", name: node.name, span: node.span },
            node.bindings.map((binding) => binding.init),
            node.span,
          ),
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

const lowerExpr = (expr: NamedLetExpr): Expr => {
  if (expr.tag === "named-let") {
    return namedLetToCall({
      ...expr,
      bindings: expr.bindings.map((binding) => ({
        name: binding.name,
        init: lowerExpr(binding.init),
      })),
      body: lowerItems(expr.body),
    });
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

function lowerNamedLetStmt(item: Stmt): Stmt {
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
      const alternative = item.alternative === null ? null : lowerNamedLetStmt(item.alternative);
      return {
        ...item,
        test: lowerExpr(item.test),
        consequent: lowerNamedLetStmt(item.consequent),
        alternative,
      };
    }
    case "while":
      return { ...item, test: lowerExpr(item.test), body: lowerNamedLetStmt(item.body) };
    case "for-of":
      return { ...item, iterable: lowerExpr(item.iterable), body: lowerNamedLetStmt(item.body) };
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

/** The evaluator's case for the derived forms: lower, then evaluate. */
export const evalWithNamedLet = (
  expr: NamedLetExpr,
  env: Env,
  session: Session = new Session("core"),
): Outcome => session.evaluate(lowerExpr(expr), env);

export function ex_4_08(): string {
  return (
    "The named binding is a local named function over the group's names, called on the " +
    "initializers: `namedLetToCall` builds `(() => { function loop(v1, ...) { body } " +
    "return loop(e1, ...); })()`, and the function declaration is what recursive calls in " +
    "the body resolve to. Fibonacci 10 by named let is 55, factorial 5 with an accumulator " +
    "is 120, plain let still answers 7 in the same evaluator, and a named let inside a " +
    "procedure body recurs through its name to 55."
  );
}
