// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.53

package sicp.ch4.solutions

import arrow.core.raise.either
import sicp.ch4.AmbEvaluator
import sicp.ch4.AmbExec
import sicp.ch4.ambDriver
import sicp.runtime.AppE
import sicp.runtime.Env
import sicp.runtime.VSym
import sicp.runtime.VarE

/** The combined evaluator: `permanent-set!` without the undo trail and
 * `if-fail` as the exhaustion guard. */
internal class WithPermanentSetIfFail(
    global: Env,
) : AmbEvaluator(global) {
    override fun reservedClause(expr: AppE): AmbExec? {
        val head = expr.operator as? VarE ?: return null
        return when (head.name) {
            "permanent-set!" -> permanentSetClause(expr)
            "if-fail" -> ifFailClause(expr)
            else -> super.reservedClause(expr)
        }
    }

    private fun permanentSetClause(expr: AppE): AmbExec {
        val name = (expr.operands[0] as VarE).name
        val vproc = analyzedOperands(expr)[1]
        return { env, succeed ->
            vproc(env) { value ->
                env.set(name, value)
                succeed(VSym("ok"))
            }
        }
    }

    private fun ifFailClause(expr: AppE): AmbExec {
        val aprocs = analyzedOperands(expr)
        return { env, succeed ->
            pushFailGuard(aprocs[1], env, succeed)
            aprocs[0](env, succeed)
        }
    }
}

private val pairsQuery: String =
    "(let ((pairs '()))\n  (if-fail\n   (let ((p (prime-sum-pair '(1 3 5 8) '(20 35 110))))\n     (permanent-set! pairs (cons p pairs))\n     (amb))\n   pairs))"

/** The three prime-sum pairs accumulate without an undo trail, the
 * closing `(amb)` fails each branch, and the guard delivers the
 * collected list. => "((8 35) (3 110) (3 20))" */
public fun pairsResult(): String =
    either {
        val driver = ambDriver({ env, _ -> WithPermanentSetIfFail(env) }, AMB_BASE_PRELUDE)
        val answer = driver.solve(pairsQuery) ?: throw AssertionError("no pairs")
        sicp.ch4.printValue(answer)
    }.fold(
        { e -> throw AssertionError(e.toString()) },
        { it },
    )
