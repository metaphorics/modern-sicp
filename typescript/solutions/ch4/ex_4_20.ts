// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.20: recursive bindings as a derived expression. The
 * `RecursiveBindings` node derives to a grouped binding whose
 * uninitialized cells the runtime pre-binds before any initializer
 * runs, so an initializer may refer to its siblings — the let-and-
 * writes shape of the book's `letrec`. A plain grouped binding differs
 * exactly there: its initializers are the call's arguments and see only
 * the outer frame. The lowered form is pure shared syntax, so it nests
 * anywhere a core expression nests.
 */
import { Session } from "../../packages/ch4/src/01-metacircular.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import {
  call,
  type Decl,
  type Expr,
  lam,
  type Stmt,
  varDecl,
} from "../../packages/ch4/src/syntax/ast.js";
import type { Span } from "../../packages/ch4/src/syntax/diagnostics.js";

/** One binding of a recursive binding: a name and its initializer. */
export interface RecursiveBinding {
  readonly name: string;
  readonly init: Expr;
}

/** The recursive binding extension node beside the shared syntax. */
export interface RecursiveNode {
  readonly tag: "recursive-bindings";
  readonly bindings: ReadonlyArray<RecursiveBinding>;
  readonly body: ReadonlyArray<Decl | Stmt>;
  readonly span: Span;
}

/** The syntax this exercise evaluates: shared expressions plus its form. */
export type RecursiveExpr = Expr | RecursiveNode;

/** Builds a recursive binding node. */
export const recursiveNode = (
  bindings: ReadonlyArray<RecursiveBinding>,
  body: ReadonlyArray<Decl | Stmt>,
  span: Span,
): RecursiveNode => ({ tag: "recursive-bindings", bindings, body, span });

/**
 * The derivation: a grouped binding with uninitialized cells — one
 * immediately called block whose declarations pre-bind every name
 * before the initializing writes run.
 */
export const recursiveToCall = (node: RecursiveNode): Expr =>
  call(
    lam(
      [],
      [
        ...node.bindings.map((binding) =>
          varDecl("let", binding.name, binding.init, null, node.span),
        ),
        ...node.body,
      ],
      node.span,
    ),
    [],
    node.span,
  );

/** The evaluator's case for the derived form: lower, then evaluate. */
export const evalWithRecursive = (
  expr: RecursiveExpr,
  env: Env,
  session: Session = new Session("core"),
): Outcome =>
  session.evaluate(expr.tag === "recursive-bindings" ? recursiveToCall(expr) : expr, env);

export function ex_4_20(): string {
  return (
    "The recursive binding derives to a grouped binding with uninitialized cells: one " +
    "immediately called block whose declarations pre-bind every name before the " +
    "initializing writes run, so initializers see their siblings. The mutually recursive " +
    "even?/odd? answers false for 5, factorial 10 answers 3628800, a nested recursive " +
    "double answers 42 for 21, and Louis's plain grouped binding fails with unbound-name " +
    "on a where the recursive analog answers 2 — plain lets evaluate their initializers " +
    "in the outer frame."
  );
}
