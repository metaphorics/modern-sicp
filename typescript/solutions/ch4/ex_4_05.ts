// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.5 (host-language replacement keeping the number and the
 * case-analysis objective): add a recipient clause to the switch case
 * analysis, the typed analog of the book's `(test => recipient)` clause.
 * A recipient is a one-argument procedure called with the matched value.
 * `switchToIf` is the 4.1.6-style transformation the book prescribes:
 * the discriminant is evaluated exactly once and bound as
 * `@@switch-value`, then the clauses become a nest of ifs comparing
 * `@@switch-value` with each case test. A recipient clause's consequent
 * is the recipient applied to `@@switch-value`; a body clause runs its
 * body. The single binding is what makes the test run once and its value
 * reach the recipient, the same shape the book recommends for the arrow
 * clause.
 */
import { Session } from "../../packages/ch4/src/01-metacircular.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { outcomeOf } from "../../packages/ch4/src/runtime/errors.js";
import {
  bin,
  block,
  call,
  type Decl,
  type Expr,
  exprStmt,
  ident,
  ifStmt,
  type Stmt,
  varDecl,
} from "../../packages/ch4/src/syntax/ast.js";
import type { Span } from "../../packages/ch4/src/syntax/diagnostics.js";

/** The generated name the discriminant is bound to exactly once. */
export const switchValueName = "@@switch-value";

/** A case clause whose match runs a recipient on the matched value. */
export interface RecipientClause {
  readonly kind: "recipient";
  readonly test: Expr;
  readonly recipient: Expr;
  readonly span: Span;
}

/** A case clause whose match runs its body. */
export interface BodyClause {
  readonly kind: "body";
  readonly test: Expr;
  readonly body: ReadonlyArray<Decl | Stmt>;
  readonly span: Span;
}

/** One clause of the extended case analysis. */
export type SwitchClause = BodyClause | RecipientClause;

/** The switch case analysis with recipient clauses. */
export interface RecipientSwitch {
  readonly tag: "switch-recipient";
  readonly discriminant: Expr;
  readonly clauses: ReadonlyArray<SwitchClause>;
  readonly defaultBody: ReadonlyArray<Decl | Stmt> | null;
  readonly span: Span;
}

/** A recipient clause: `case test => recipient`. */
export const recipientClause = (
  test: Expr,
  recipient: Expr,
  span: Span = test.span,
): RecipientClause => ({
  kind: "recipient",
  test,
  recipient,
  span,
});

/** A body clause: `case test: body`. */
export const bodyClause = (
  test: Expr,
  body: ReadonlyArray<Decl | Stmt>,
  span: Span = test.span,
): BodyClause => ({ kind: "body", test, body, span });

/** The extended case analysis over one discriminant. */
export const recipientSwitch = (
  discriminant: Expr,
  clauses: ReadonlyArray<SwitchClause>,
  defaultBody: ReadonlyArray<Decl | Stmt> | null = null,
): RecipientSwitch => ({
  tag: "switch-recipient",
  discriminant,
  clauses,
  defaultBody,
  span: discriminant.span,
});

// ---------------------------------------------------------------------
// 4.1.2 syntax procedures
// ---------------------------------------------------------------------

/** The discriminant expression of the analysis. */
export const switchDiscriminant = (node: RecipientSwitch): Expr => node.discriminant;

/** The clauses of the analysis, in source order. */
export const switchClauses = (node: RecipientSwitch): ReadonlyArray<SwitchClause> => node.clauses;

/** The test of one clause. */
export const clauseTest = (clause: SwitchClause): Expr => clause.test;

/** Whether this clause carries a recipient instead of a body. */
export const isRecipientClause = (clause: SwitchClause): clause is RecipientClause =>
  clause.kind === "recipient";

/** The recipient of a recipient clause. */
export const clauseRecipient = (clause: RecipientClause): Expr => clause.recipient;

// ---------------------------------------------------------------------
// switchToIf and evaluation
// ---------------------------------------------------------------------

const chainToIf = (
  clauses: ReadonlyArray<SwitchClause>,
  index: number,
  fallback: Stmt | null,
): Stmt | null => {
  const clause = clauses[index];
  if (clause === undefined) {
    return fallback;
  }
  const test = bin("===", ident(switchValueName, clause.span), clause.test, clause.span);
  const consequent = isRecipientClause(clause)
    ? exprStmt(
        call(clause.recipient, [ident(switchValueName, clause.span)], clause.span),
        clause.span,
      )
    : { tag: "block" as const, body: clause.body, span: clause.span };
  return ifStmt(test, consequent, chainToIf(clauses, index + 1, fallback), clause.span);
};

/**
 * The transformation the book prescribes: bind the discriminant once as
 * `@@switch-value`, then nest one `if` per clause, comparing the bound
 * value with each case test. A recipient clause applies its recipient to
 * the bound value; a body clause runs its body; the default body is the
 * final `else`.
 */
export const switchToIf = (node: RecipientSwitch): Stmt => {
  const fallback: Stmt | null =
    node.defaultBody === null
      ? exprStmt({ tag: "undefined", span: node.span }, node.span)
      : { tag: "block", body: node.defaultBody, span: node.span };
  const chain = chainToIf(node.clauses, 0, fallback);
  const items: Array<Decl | Stmt> = [
    varDecl("const", switchValueName, node.discriminant, null, node.span),
  ];
  if (chain !== null) {
    items.push(chain);
  }
  return { tag: "block", body: items, span: node.span };
};

/** Evaluates the extended analysis through its `switchToIf` lowering. */
export const evalWithRecipientSwitch = (
  node: RecipientSwitch,
  env: Env,
  session: Session = new Session("core"),
): Outcome => outcomeOf(session.execStatement(switchToIf(node), env));

export function ex_4_05(): string {
  return (
    "The recipient clause is added to the switch case analysis: when a case test matches, " +
    "a one-argument recipient procedure is called with the matched value. `switchToIf` " +
    "binds the discriminant exactly once as `@@switch-value` and nests the clause tests " +
    "as ifs, so the discriminant expression runs once and its value reaches the " +
    'recipient — over `entries = [{key:"a",value:1},{key:"b",value:2}]`, the analysis ' +
    'of `lookup("b", entries)` with recipient `entryValue` answers 2, `lookup("a", ...)` ' +
    "answers 1, and an unmatched discriminant falls to the default body."
  );
}
