// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { ExecutionProcedure } from "../../packages/ch4/src/01-metacircular.js";
/**
 * Exercise 4.22: `let` in the analyzed evaluator. A grouped binding is
 * a derived expression, so `analyzeLet` analyzes the combination it
 * stands for: `call(lam(params, bodyDecls), inits)` — exactly the
 * direct evaluator's `letToCall` target, which is why this file
 * borrows that derivation rather than spelling a second one. The
 * analyzed form therefore equals the direct form, closures made by the
 * analyzed lambda store their body's execution procedure, and running
 * the analysis needs no let machinery at run time.
 */
import { analyze, Session } from "../../packages/ch4/src/01-metacircular.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { type LetExpr, type LetNode, letToCall, lowerLetExpr } from "./ex_4_06.js";

/**
 * The analyzed evaluator's let case: lower to the call form and
 * delegate to `analyze`.
 */
export const analyzeLet = (node: LetNode): ExecutionProcedure => analyze(letToCall(node));

/** Analyze-and-run over syntax that may carry grouped bindings. */
export const evalAnalyzedLet = (
  expr: LetExpr,
  env: Env,
  session: Session = new Session("core"),
): Outcome => analyze(lowerLetExpr(expr))(env);

export function ex_4_22(): string {
  return (
    "The analyzed evaluator's let case lowers to `call(lam(params, bodyDecls), inits)` — " +
    "the direct evaluator's `letToCall` target — and delegates to `analyze`, so the " +
    "analyzed form equals the direct form and no let machinery exists at run time. " +
    "Through analysis, `(let x = 3) x + 4` answers 7, the nested-body f answers 20 for 4, " +
    "nested grouped bindings answer 12 exactly as the fully derived hand-written " +
    "combination does, and the execution procedure from analyzeLet answers 7 on a fresh " +
    "environment."
  );
}
