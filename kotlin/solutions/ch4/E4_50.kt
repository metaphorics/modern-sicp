// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.50

package sicp.ch4.solutions

import arrow.core.raise.either
import sicp.ch4.AmbEvaluator
import sicp.ch4.AmbExec
import sicp.ch4.ambDriver
import sicp.ch4.printValue
import sicp.runtime.AppE
import sicp.runtime.Env
import sicp.runtime.Random
import sicp.runtime.VarE

/** The `ramb` special form: `amb` with the alternatives visited in the
 * seeded xorshift's order. The seed rides in through the driver (D31);
 * [visitOrder] shuffles one choice-point visit Fisher-Yates style, so a
 * fixed seed makes the session reproducible. */
internal class WithRamb(
    global: Env,
    random: Random?,
) : AmbEvaluator(global, random) {
    override fun reservedClause(expr: AppE): AmbExec? {
        val head = expr.operator as? VarE ?: return null
        if (head.name == "ramb") {
            return analyzedAmb(analyzedOperands(expr))
        }
        return super.reservedClause(expr)
    }

    override fun visitOrder(alternatives: List<AmbExec>): List<AmbExec> {
        val rng = random ?: throw IllegalStateException("ramb needs a seeded driver")
        val shuffled = alternatives.toMutableList()
        for (i in shuffled.size - 1 downTo 1) {
            val j = rng.random(i + 1L).toInt()
            val held = shuffled[i]
            shuffled[i] = shuffled[j]
            shuffled[j] = held
        }
        return shuffled
    }
}

private val seed: ULong = 20260925UL

/** The seeded shuffle's enumeration of one five-way choice.
 * => [(3), (2), (5), (1), (4)] */
public fun rambEnumeration(): List<String> =
    either {
        val driver = ambDriver(::WithRamb, AMB_BASE_PRELUDE, seed)
        answerLines(driver, "(list (ramb 1 2 3 4 5))")
    }.fold(
        { e -> throw AssertionError(e.toString()) },
        { it },
    )

/** Mixed into Alyssa's generator, ramb escapes the first-alternative
 * recursion that made 4.49's sentences boring: the first sentence
 * already picks words the depth-first search never reached. */
public fun rambGeneratedFirst(): String =
    either {
        val driver =
            ambDriver(
                ::WithRamb,
                "$AMB_BASE_PRELUDE\n$PARSER_PROGRAM\n$RAMB_GENERATOR_PROGRAM",
                seed,
            )
        printValue(driver.solve("(parse '(any input at all))") ?: throw AssertionError("no sentence"))
    }.fold(
        { e -> throw AssertionError(e.toString()) },
        { it },
    )
