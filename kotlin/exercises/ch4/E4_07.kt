// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.7

package sicp.ch4.exercises

import arrow.core.raise.Raise
import sicp.ch4.EvalStep
import sicp.ch4.Evaluator
import sicp.runtime.AppE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.PendingSolution
import sicp.runtime.SchemeError

/**
 * Exercise 4.7: `let*` as nested `let`s. [letStarToNestedLets] folds the
 * bindings right to left, `(let* ((x 3) (y (+ x 1))) body)` =>
 * `(let ((x 3)) (let ((y (+ x 1))) body))`, so each init sees the earlier
 * bindings of the same `let*`. Adding the one `step` clause is sufficient:
 * every derived `let` re-enters the evaluator at the next depth, which is
 * also what makes a `let*` nested inside another's body work.
 */
public class WithLetStar(
    global: Env,
) : Evaluator(global) {
    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep = throw PendingSolution()
}

/** The book's `let*->nested-lets`, folded right to left; the parsed
 * `let*` arrives as the application the parser leaves for it. */
context(r: Raise<SchemeError>)
public fun letStarToNestedLets(expr: AppE): Expr = throw PendingSolution()
