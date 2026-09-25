// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.9

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
 * Exercise 4.9: iteration constructs designed as derived expressions.
 * `(while test body...)` and `(until test body...)` each rewrite to a
 * set!-installed zero-argument loop procedure,
 * `((lambda (tag) (set! tag (lambda () round)) (tag)) 'tag)` with the round
 * being `(if test (begin body... (tag)) ())` for `while` and the mirrored
 * `(if test () (begin body... (tag)))` for `until`. The rewrite fixes both
 * properties that matter: the body re-enters the full evaluator at every
 * iteration, and the self-call sits in tail position, so the loop runs an
 * iterative process in constant host stack. A while summing 1 to 5 answers
 * 15.
 */
public class WithLoops(
    global: Env,
) : Evaluator(global) {
    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep = throw PendingSolution()
}

/** `(while test body...)`: repeat the body while the test holds. */
context(r: Raise<SchemeError>)
public fun whileToCombination(expr: AppE): Expr = throw PendingSolution()

/** `(until test body...)`: repeat the body until the test holds. */
context(r: Raise<SchemeError>)
public fun untilToCombination(expr: AppE): Expr = throw PendingSolution()
