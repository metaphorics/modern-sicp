// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.2

package sicp.ch4.solutions

import arrow.core.raise.Raise
import kotlinx.collections.immutable.persistentListOf
import kotlinx.collections.immutable.toPersistentList
import sicp.ch4.EvalStep
import sicp.ch4.Evaluator
import sicp.runtime.AppE
import sicp.runtime.DefineE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.SchemeError
import sicp.runtime.SetE
import sicp.runtime.VarE

/**
 * Exercise 4.2, part (a): Louis Reasoner's applications-first eval checks
 * applications before assignments and definitions, so a definition
 * evaluates as an application of the (unbound) operator `define`:
 * `(define x 3)` => Error: unbound variable: define. This host's parser
 * classifies the two forms into typed [DefineE]/[SetE] nodes before eval
 * sees them, so the reordering arrives as a demotion: the node is rebuilt
 * as the pair Louis's clause order catches, and the application clause runs
 * before any special-form dispatch.
 */
public class ApplicationsFirst(
    global: Env,
) : Evaluator(global) {
    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep =
        when (expr) {
            is DefineE -> EvalStep.Continue(AppE(VarE("define"), persistentListOf(VarE(expr.name), expr.value)), env)
            is SetE -> EvalStep.Continue(AppE(VarE("set!"), persistentListOf(VarE(expr.name), expr.value)), env)
            is AppE -> evalApplication(expr, env)
            else -> super.step(expr, env)
        }
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
    ): EvalStep =
        if (expr is AppE && expr.operator == VarE("call") && expr.operands.isNotEmpty()) {
            EvalStep.Continue(AppE(expr.operands.first(), expr.operands.drop(1).toPersistentList()), env)
        } else {
            super.step(expr, env)
        }
}
