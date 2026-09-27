// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.54

package sicp.ch4.solutions

import arrow.core.raise.either
import sicp.ch4.AmbEvaluator
import sicp.ch4.AmbExec
import sicp.ch4.AmbFail
import sicp.ch4.ambDriver
import sicp.runtime.AppE
import sicp.runtime.Env
import sicp.runtime.VSym
import sicp.runtime.VarE
import sicp.runtime.isTrue

/** `require` as a special form: the completed `analyze-require`. The
 * predicate's success continuation fails outright on a false value --
 * the same dead end the user-defined `(define (require p) ...)` reaches
 * through `(amb)`, without the procedure call. */
internal class WithRequireSpecial(
    global: Env,
) : AmbEvaluator(global) {
    override fun reservedClause(expr: AppE): AmbExec? {
        val head = expr.operator as? VarE ?: return null
        if (head.name != "require") {
            return super.reservedClause(expr)
        }
        val pproc = analyzedOperands(expr)[0]
        return { env, succeed ->
            pproc(env) { predValue ->
                if (isTrue(predValue)) succeed(VSym("ok")) else throw AmbFail
            }
        }
    }
}

/** The evenness filter over a five-way choice: the false predicates
 * prune 1, 3, and 5, so the answers are 2 then 4. */
public fun requireFilteredEvens(): List<String> =
    either {
        val driver = ambDriver({ env, _ -> WithRequireSpecial(env) }, AMB_BASE_PRELUDE)
        answerLines(driver, "(let ((x (amb 1 2 3 4 5)))\n  (require (even? x))\n  x)")
    }.fold(
        { e -> throw AssertionError(e.toString()) },
        { it },
    )

/** A satisfied requirement answers ok. */
public fun requireSatisfied(): String =
    either {
        val driver = ambDriver({ env, _ -> WithRequireSpecial(env) }, AMB_BASE_PRELUDE)
        sicp.ch4.printValue(driver.solve("(require (= 1 1))") ?: throw AssertionError("no answer"))
    }.fold(
        { e -> throw AssertionError(e.toString()) },
        { it },
    )
