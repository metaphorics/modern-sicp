// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.40

package sicp.ch4.solutions

import arrow.core.raise.either
import sicp.ch4.AmbEvaluator
import sicp.runtime.Env
import sicp.runtime.Random

private val plain: (Env, Random?) -> AmbEvaluator = { env, _ -> AmbEvaluator(env) }

/** Assignments of five people to five floors with no restrictions:
 * 5^5. => 3125 */
public fun assignmentsBeforeDistinct(): Int =
    either {
        val driver = newDriver(plain, "$AMB_BASE_PRELUDE\n$DWELLING_SPACE_PROGRAM")
        driver.solveAll("(dwellings-all)").size
    }.fold(
        { e -> throw AssertionError(e.toString()) },
        { it },
    )

/** The same space after the distinctness requirement: 5!. => 120 */
public fun assignmentsAfterDistinct(): Int =
    either {
        val driver = newDriver(plain, "$AMB_BASE_PRELUDE\n$DWELLING_SPACE_PROGRAM")
        driver.solveAll("(dwellings-distinct)").size
    }.fold(
        { e -> throw AssertionError(e.toString()) },
        { it },
    )

/** The unprompted program's backtracks to the first answer: 1470. */
public fun naiveBacktracksToFirst(): Long = backtracksToFirst(plain, "$AMB_BASE_PRELUDE\n$DWELLING_PROGRAM", "(multiple-dwelling)")

/** The pruned nest's backtracks to the first answer: 210. */
public fun prunedBacktracksToFirst(): Long =
    backtracksToFirst(plain, "$AMB_BASE_PRELUDE\n$DWELLING_PRUNED_PROGRAM", "(multiple-dwelling-faster)")

/** The pruned program's answer, the same assignment. */
public fun prunedAnswer(): String = firstAnswerLine(plain, "$AMB_BASE_PRELUDE\n$DWELLING_PRUNED_PROGRAM", "(multiple-dwelling-faster)")
