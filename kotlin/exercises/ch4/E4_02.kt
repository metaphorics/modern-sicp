// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.2

package sicp.ch4.exercises

import arrow.core.raise.Raise
import sicp.ch4.EvalStep
import sicp.ch4.Evaluator
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.PendingSolution
import sicp.runtime.SchemeError

/**
 * Exercise 4.2, part (a): Louis Reasoner's applications-first eval checks
 * applications before assignments and definitions, so a definition
 * evaluates as an application of the (unbound) operator `define`:
 * `(define x 3)` => Error: unbound variable: define. This host's parser
 * classifies the two forms into typed nodes before eval sees them, so the
 * reordering arrives as a demotion: rebuild the node as the pair Louis's
 * clause order catches, and check applications before any special-form
 * dispatch.
 */
public class ApplicationsFirst(
    global: Env,
) : Evaluator(global) {
    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep = throw PendingSolution()
}

/**
 * Exercise 4.2, part (b): application syntax grows a distinguished `call`
 * head, `(call f x)`, and every other pair applies as before. Stripping the
 * sugar in `step` -- one clause, re-entered at every depth -- keeps the
 * special forms and bare combinations working: `(call (lambda (x) (* x x))
 * 7)` => 49.
 */
public class CallSyntax(
    global: Env,
) : Evaluator(global) {
    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep = throw PendingSolution()
}
