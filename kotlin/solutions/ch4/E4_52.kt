// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.52

package sicp.ch4.solutions

import sicp.ch4.AmbEvaluator
import sicp.ch4.AmbExec
import sicp.ch4.ambDriver
import sicp.runtime.AppE
import sicp.runtime.Env
import sicp.runtime.VarE

/** `if-fail`: the alternative answers only when every choice of the
 * first expression is exhausted. The guard is a frame pushed under the
 * first expression's choice frames, so it fires exactly when the search
 * unwinds past them -- and its trail mark rolls the failed branch's
 * assignments back. */
internal class WithIfFail(
    global: Env,
) : AmbEvaluator(global) {
    override fun reservedClause(expr: AppE): AmbExec? {
        val head = expr.operator as? VarE ?: return null
        if (head.name != "if-fail") {
            return super.reservedClause(expr)
        }
        val aprocs = analyzedOperands(expr)
        return { env, succeed ->
            pushFailGuard(aprocs[1], env, succeed)
            aprocs[0](env, succeed)
        }
    }
}

private val allOddQuery: String =
    "(if-fail (let ((x (an-element-of '(1 3 5))))\n           (require (even? x))\n           x)\n         'all-odd)"

private val eightQuery: String =
    "(if-fail (let ((x (an-element-of '(1 3 5 8))))\n           (require (even? x))\n           x)\n         'all-odd)"

/** The all-odd session: the alternative answers, then exhaustion. */
public fun ifFailAllOddTranscript(): String {
    val driver = ambDriver({ env, _ -> WithIfFail(env) }, AMB_BASE_PRELUDE)
    return driver.input(allOddQuery) + driver.input("try-again")
}

/** The session with 8 available: 8 answers, `all-odd` on try-again,
 * then exhaustion. */
public fun ifFailEightTranscript(): String {
    val driver = ambDriver({ env, _ -> WithIfFail(env) }, AMB_BASE_PRELUDE)
    return driver.input(eightQuery) + driver.input("try-again") + driver.input("try-again")
}
