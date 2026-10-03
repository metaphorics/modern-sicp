// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.10: one evaluator, two surface syntaxes. The special forms
 * move into a syntax table: a map from marker name to a handler that
 * receives the marker call's arguments and builds a shared-syntax node.
 * The evaluator knows nothing about markers — it receives the same
 * `Expr` data either way — so `evaluate` and `applyProcedure` are
 * untouched. Two tables are installed: the standard marker names, and
 * the same handlers under names spelled backwards. A marker from the
 * other table is just a name: the call stays an application and fails
 * as an unbound name at evaluation.
 */
import { Session } from "../../packages/ch4/src/01-metacircular.js";
import { read, readAll } from "../../packages/ch4/src/read.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { outcomeOf } from "../../packages/ch4/src/runtime/errors.js";
import {
  assign,
  type CaseClause,
  call,
  cond,
  type Decl,
  type Expr,
  exprStmt,
  ident,
  lam,
  type ObjectField,
  param,
  returnStmt,
  type Stmt,
} from "../../packages/ch4/src/syntax/ast.js";
import type { Span } from "../../packages/ch4/src/syntax/diagnostics.js";

/** One installed form: marker arguments in, shared syntax out. */
export type MarkerHandler = (args: ReadonlyArray<Expr>, span: Span) => Expr;

/** The syntax table of one surface: marker name to handler. */
export class SyntaxTable {
  readonly #handlers = new Map<string, MarkerHandler>();

  /** Installs one handler under its marker name. */
  put(name: string, handler: MarkerHandler): void {
    this.#handlers.set(name, handler);
  }

  /** The handler installed under `name`, if any. */
  get(name: string): MarkerHandler | undefined {
    return this.#handlers.get(name);
  }
}

const at = (args: ReadonlyArray<Expr>, index: number, span: Span): Expr =>
  args[index] ?? { tag: "undefined", span };

/** `fn("x", "y", body)` — a function over the named parameters. */
const lambdaMarker: MarkerHandler = (args, span) => {
  const names: string[] = [];
  let index = 0;
  while (index < args.length - 1) {
    const arg = args[index];
    if (arg === undefined || arg.tag !== "string") {
      break;
    }
    names.push(arg.value);
    index += 1;
  }
  return lam(
    names.map((name) => param(name, null, span)),
    [exprStmt(at(args, index, span), span)],
    span,
  );
};

/** `branch(test, consequent, alternative)` — the conditional. */
const branchMarker: MarkerHandler = (args, span) =>
  cond(at(args, 0, span), at(args, 1, span), at(args, 2, span), span);

/** `seq(a, b, ..., last)` — the sequence, answering its last form. */
const sequenceMarker: MarkerHandler = (args, span) => {
  const last =
    args.length === 0 ? { tag: "undefined" as const, span } : at(args, args.length - 1, span);
  const before = args.slice(0, -1).map((arg) => exprStmt(arg, span));
  return call(lam([], [...before, returnStmt(last, span)], span), [], span);
};

/** `set("x", value)` — the assignment. */
const assignmentMarker: MarkerHandler = (args, span) => {
  const target = at(args, 0, span);
  const name = target.tag === "string" ? target.value : "@@bad-target";
  return assign(ident(name, span), at(args, 1, span), span);
};

/** `pick(t1, v1, t2, v2, ..., fallback)` — the cond chain. */
const pickMarker: MarkerHandler = (args, span) => {
  const build = (index: number): Expr => {
    const test = args[index];
    const value = args[index + 1];
    const rest = args[index + 2];
    if (test === undefined || value === undefined) {
      return { tag: "undefined", span };
    }
    return rest === undefined ? value : cond(test, value, build(index + 2), span);
  };
  return build(0);
};

/** The standard surface: the marker names the book's forms read with. */
export const makeStandardSyntax = (): SyntaxTable => {
  const table = new SyntaxTable();
  table.put("fn", lambdaMarker);
  table.put("branch", branchMarker);
  table.put("seq", sequenceMarker);
  table.put("set", assignmentMarker);
  table.put("pick", pickMarker);
  return table;
};

/** The second surface: the same handlers under names spelled backwards. */
export const makeReversedSyntax = (): SyntaxTable => {
  const table = new SyntaxTable();
  const standard = makeStandardSyntax();
  for (const name of ["fn", "branch", "seq", "set", "pick"]) {
    const handler = standard.get(name);
    if (handler !== undefined) {
      table.put(name.split("").reverse().join(""), handler);
    }
  }
  return table;
};

// ---------------------------------------------------------------------
// Reading one surface into the shared syntax
// ---------------------------------------------------------------------

const lowerExpr = (expr: Expr, table: SyntaxTable): Expr => {
  if (expr.tag === "call" && expr.callee.tag === "variable") {
    const handler = table.get(expr.callee.name);
    const args = expr.args.map((arg) => lowerExpr(arg.expr, table));
    if (handler !== undefined) {
      return handler(args, expr.span);
    }
    return {
      ...expr,
      callee: lowerExpr(expr.callee, table),
      args: expr.args.map((arg, index) => ({ kind: arg.kind, expr: args[index] ?? arg.expr })),
    };
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
      return { ...expr, exprs: expr.exprs.map((inner) => lowerExpr(inner, table)) };
    case "array":
      return {
        ...expr,
        elements: expr.elements.map((arg) => ({
          kind: arg.kind,
          expr: lowerExpr(arg.expr, table),
        })),
      };
    case "object": {
      const fields: ObjectField[] = expr.fields.map((field) => ({
        key: field.key,
        value: lowerExpr(field.value, table),
        span: field.span,
      }));
      return { ...expr, fields };
    }
    case "unary":
      return { ...expr, operand: lowerExpr(expr.operand, table) };
    case "binary":
      return { ...expr, left: lowerExpr(expr.left, table), right: lowerExpr(expr.right, table) };
    case "logical":
      return { ...expr, left: lowerExpr(expr.left, table), right: lowerExpr(expr.right, table) };
    case "conditional":
      return {
        ...expr,
        test: lowerExpr(expr.test, table),
        consequent: lowerExpr(expr.consequent, table),
        alternative: lowerExpr(expr.alternative, table),
      };
    case "permanent-assign":
    case "assign":
      return {
        ...expr,
        target: lowerExpr(expr.target, table),
        value: lowerExpr(expr.value, table),
      };
    case "if-fail":
      return {
        ...expr,
        expression: lowerExpr(expr.expression, table),
        fallback: lowerExpr(expr.fallback, table),
      };
    case "arrow":
      return { ...expr, body: { body: lowerItems(expr.body.body, table), span: expr.body.span } };
    case "call":
      return {
        ...expr,
        callee: lowerExpr(expr.callee, table),
        args: expr.args.map((arg) => ({ kind: arg.kind, expr: lowerExpr(arg.expr, table) })),
      };
    case "member":
      return { ...expr, object: lowerExpr(expr.object, table) };
    case "index":
      return {
        ...expr,
        object: lowerExpr(expr.object, table),
        index: lowerExpr(expr.index, table),
      };
    case "new-error":
      return { ...expr, args: expr.args.map((inner) => lowerExpr(inner, table)) };
    case "new-map":
      return { ...expr, args: expr.args.map((inner) => lowerExpr(inner, table)) };
    case "new-set":
      return { ...expr, args: expr.args.map((inner) => lowerExpr(inner, table)) };
    case "delay":
      return { ...expr, expr: lowerExpr(expr.expr, table) };
    case "force":
      return { ...expr, expr: lowerExpr(expr.expr, table) };
    case "require":
      return { ...expr, condition: lowerExpr(expr.condition, table) };
    case "choose":
      return { ...expr, alternatives: expr.alternatives.map((inner) => lowerExpr(inner, table)) };
    case "ramb":
      return { ...expr, alternatives: expr.alternatives.map((inner) => lowerExpr(inner, table)) };
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

function lowerSyntaxStmt(item: Stmt, table: SyntaxTable): Stmt {
  const lowered = lowerItem(item, table);
  if (!isStmt(lowered)) {
    throw new Error("statement lowering produced a declaration");
  }
  return lowered;
}
const lowerItem = (item: Decl | Stmt, table: SyntaxTable): Decl | Stmt => {
  switch (item.tag) {
    case "import":
    case "type-decl":
    case "interface-decl":
    case "break":
    case "continue":
      return item;
    case "var-decl":
      return { ...item, init: lowerExpr(item.init, table) };
    case "function-decl":
      return { ...item, body: { body: lowerItems(item.body.body, table), span: item.body.span } };
    case "expr-stmt":
      return { ...item, expr: lowerExpr(item.expr, table) };
    case "return":
      return item.argument === null ? item : { ...item, argument: lowerExpr(item.argument, table) };
    case "throw":
      return { ...item, argument: lowerExpr(item.argument, table) };
    case "if": {
      const alternative =
        item.alternative === null ? null : lowerSyntaxStmt(item.alternative, table);
      return {
        ...item,
        test: lowerExpr(item.test, table),
        consequent: lowerSyntaxStmt(item.consequent, table),
        alternative,
      };
    }
    case "while":
      return {
        ...item,
        test: lowerExpr(item.test, table),
        body: lowerSyntaxStmt(item.body, table),
      };
    case "for-of":
      return {
        ...item,
        iterable: lowerExpr(item.iterable, table),
        body: lowerSyntaxStmt(item.body, table),
      };
    case "block":
      return { ...item, body: lowerItems(item.body, table) };
    case "switch": {
      const cases: CaseClause[] = item.cases.map((clause) => ({
        test: lowerExpr(clause.test, table),
        body: lowerItems(clause.body, table),
        span: clause.span,
      }));
      return {
        ...item,
        discriminant: lowerExpr(item.discriminant, table),
        cases,
        defaultBody: item.defaultBody === null ? null : lowerItems(item.defaultBody, table),
      };
    }
    case "try": {
      const blockOf = (body: {
        readonly body: ReadonlyArray<Decl | Stmt>;
        readonly span: Span;
      }): {
        readonly body: ReadonlyArray<Decl | Stmt>;
        readonly span: Span;
      } => ({ body: lowerItems(body.body, table), span: body.span });
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

const lowerItems = (
  items: ReadonlyArray<Decl | Stmt>,
  table: SyntaxTable,
): ReadonlyArray<Decl | Stmt> => items.map((item) => lowerItem(item, table));

/** Reads one form in the given surface and evaluates it unchanged. */
export const evalFormIn = (
  table: SyntaxTable,
  text: string,
  env: Env,
  session: Session = new Session("core"),
): Outcome => session.evaluate(lowerExpr(read(text), table), env);

/** Reads a program in the given surface and runs it unchanged. */
export const evalProgramIn = (
  table: SyntaxTable,
  text: string,
  env: Env,
  session: Session = new Session("core"),
): Outcome => outcomeOf(session.execSequence(lowerItems(readAll(text), table), env));

export function ex_4_10(): string {
  return (
    "The special forms move into a syntax table keyed by marker name, so one evaluator " +
    "runs both surfaces: the square program answers 49 with `fn` and with `nf`, and " +
    "`branch(3 > 2, 1, 0)` answers 1 with `branch` and with `hcnarb`. A marker from the " +
    "other table is just a name — the call stays an application and fails as an unbound " +
    "name — and procedure bodies follow the table of the evaluator running them. " +
    "`evaluate` and `applyProcedure` never change."
  );
}
