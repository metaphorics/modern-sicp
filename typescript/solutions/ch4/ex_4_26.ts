// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { RunResult } from "../../packages/ch4/src/01-metacircular.js";
/**
 * Exercise 4.26: unless as a special form, the debate. Ben's side
 * derives unless on the strict base evaluator: a rewrite expands every
 * `unless` node into the conditional it stands for before evaluation
 * starts, because the evaluator's bodies run through their own dispatch
 * and would never see a per-form hook. It works over armed calls and
 * composes with ordinary mapping, but the name stays syntax — it is
 * not a value. Alyssa's side keeps unless an ordinary procedure under
 * the lazy experiment: the arms delay, the same calls answer, and the
 * procedure is first-class, so it is stored in an array and applied
 * from the retrieved position, which no special form can be.
 */
import { Session } from "../../packages/ch4/src/01-metacircular.js";
import { runLazySource } from "../../packages/ch4/src/02-lazy.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { cond, type Expr, num } from "../../packages/ch4/src/syntax/ast.js";
import type { Span } from "../../packages/ch4/src/syntax/diagnostics.js";

/** The `unless` extension node beside the shared syntax. */
export interface UnlessNode {
  readonly tag: "unless";
  readonly test: Expr;
  readonly usual: Expr;
  readonly exceptional: Expr;
  readonly span: Span;
}

/** The syntax Ben's derivation accepts: shared expressions plus unless. */
export type UnlessExpr = Expr | UnlessNode;

/** Builds an unless node: `(unless test usual exceptional)`. */
export const unlessNode = (
  test: Expr,
  usual: Expr,
  exceptional: Expr,
  span: Span = test.span,
): UnlessNode => ({
  tag: "unless",
  test,
  usual,
  exceptional,
  span,
});

/** Ben's derivation: `(unless c u e)` becomes `c ? e : u`. */
export const unlessToIf = (expr: UnlessExpr): Expr =>
  expr.tag === "unless" ? cond(expr.test, expr.exceptional, expr.usual, expr.span) : expr;

/** Alyssa's side: unless as an ordinary lazy procedure, first-class. */
export const lazyUnlessSource = `
const unlessL = (c: boolean, unchosen: number, chosen: number): number =>
  c ? force(chosen) : force(unchosen);
console.log(unlessL(true, delay(0), delay(42)));
console.log([unlessL(false, delay(0), delay(7)), unlessL(true, delay(0), delay(7))]);
const choices = [unlessL];
const firstChoice = choices[0];
if (firstChoice !== undefined) {
  console.log(firstChoice(false, delay(7), delay(0)));
}
`;

/** Alyssa's session through the named lazy experiment. */
export const alyssaSession = (): RunResult =>
  runLazySource(lazyUnlessSource, "lazy-memoized-experiment");

/** Ben's side runs the derived form on the strict base evaluator. */
export const benDerived = (
  expr: UnlessExpr,
  env: Env,
  session: Session = new Session("core"),
): Outcome => session.evaluate(unlessToIf(expr), env);

export function ex_4_26(): string {
  return (
    "Ben's derivation expands unless into the conditional before evaluation, so the armed " +
    "call answers 42 without touching its unused arm and the mapping answers [0, 7] — but " +
    "the name stays syntax and reading it fails with unbound-name. Alyssa's lazy procedure " +
    "answers the same calls and stays first-class: stored in an array and applied from the " +
    "retrieved position, it answers 7 where no special form could be used as a value."
  );
}
