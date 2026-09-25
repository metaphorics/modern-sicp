// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.51

package sicp.ch4.solutions

import sicp.ch4.AmbEvaluator
import sicp.ch4.AmbExec
import sicp.ch4.ambDriver
import sicp.runtime.AppE
import sicp.runtime.Env
import sicp.runtime.VSym
import sicp.runtime.VarE

/** `permanent-set!`: the assignment commits with no undo trail entry,
 * so backtracking never rolls it back. */
internal class WithPermanentSet(
    global: Env,
) : AmbEvaluator(global) {
    override fun reservedClause(expr: AppE): AmbExec? {
        val head = expr.operator as? VarE ?: return null
        if (head.name != "permanent-set!") {
            return super.reservedClause(expr)
        }
        val name = (expr.operands[0] as VarE).name
        val vproc = analyzedOperands(expr)[1]
        return { env, succeed ->
            vproc(env) { value ->
                env.set(name, value)
                succeed(VSym("ok"))
            }
        }
    }
}

private val countingPrelude: String = "$AMB_BASE_PRELUDE\n(define count 0)"

private fun countingQuery(form: String): String =
    "(let ((x (an-element-of '(a b c)))\n      (y (an-element-of '(a b c))))\n  $form\n  (require (not (eq? x y)))\n  (list x y count))"

/** The `permanent-set!` session: every trial accumulates, and the
 * counter the driver reads back afterward keeps the total 4. */
public fun permanentSetTranscript(): String {
    val driver = ambDriver({ env, _ -> WithPermanentSet(env) }, countingPrelude)
    return driver.input(countingQuery("(permanent-set! count (+ count 1))")) +
        driver.input("try-again") +
        driver.input("try-again") +
        driver.input("count")
}

/** The `set!` session: each failed trial is rolled back, every answer
 * shows its own branch's single count, and the counter reads 0 once the
 * problem ends and the trail unwinds. */
public fun setBangTranscript(): String {
    val driver = ambDriver(::AmbEvaluator, countingPrelude)
    return driver.input(countingQuery("(set! count (+ count 1))")) +
        driver.input("try-again") +
        driver.input("try-again") +
        driver.input("count")
}
