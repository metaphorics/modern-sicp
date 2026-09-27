// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.4

package sicp.ch4.exercises

import arrow.core.raise.Raise
import sicp.ch4.EvalStep
import sicp.ch4.Evaluator
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.PendingSolution
import sicp.runtime.SchemeError

/**
 * Exercise 4.4: `and` and `or` as special forms, evaluated directly so the
 * operands re-enter the full evaluator at every depth. Short circuit is the
 * specification: the first false operand stops `and` at #f, the first true
 * value is `or`'s answer, and a later operand that would raise never
 * evaluates. `(and)` is #t, `(or)` is #f, `(and 1 2 3)` is 3, and
 * `(and 1 #f (error "x"))` is #f with no fault raised.
 */
public class WithAndOr(
    global: Env,
) : Evaluator(global) {
    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep = throw PendingSolution()
}
