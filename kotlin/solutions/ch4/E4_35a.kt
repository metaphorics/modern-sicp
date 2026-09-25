// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.35a

package sicp.ch4.solutions

import arrow.core.raise.either
import sicp.ch4.AmbEvaluator
import sicp.ch4.ambDriver

/** The counting rule: every delivery of an alternative from a choice
 * point counts one choice -- the first alternative on entering the
 * choice point, and each later alternative a failure resumption
 * delivers. The engine's counter is cumulative, so each element of the
 * answer is the total choices taken by the time that triple is
 * delivered; a triple's own share is the delta from the previous
 * element (1386, then 1319, 260, 905, 194, 701).
 *
 * => [1386, 2705, 2965, 3870, 4064, 4765] */
public fun choicesTakenWithin20(): List<Long> =
    either {
        val driver = ambDriver(::AmbEvaluator, "$AMB_BASE_PRELUDE\n$TRIPLES_PROGRAM")
        val totals = mutableListOf<Long>()
        var next = driver.solve("(a-pythagorean-triple-between 1 20)")
        while (next != null) {
            totals.add(driver.evaluator.choicesTaken)
            next = driver.tryAgain()
        }
        totals
    }.fold(
        { e -> throw AssertionError(e.toString()) },
        { it },
    )

/** The single triple between 1 and 9 arrives after 330 choices.
 * => [330] */
public fun choicesTakenWithin9(): List<Long> =
    either {
        val driver = ambDriver(::AmbEvaluator, "$AMB_BASE_PRELUDE\n$TRIPLES_PROGRAM")
        val totals = mutableListOf<Long>()
        var next = driver.solve("(a-pythagorean-triple-between 1 9)")
        while (next != null) {
            totals.add(driver.evaluator.choicesTaken)
            next = driver.tryAgain()
        }
        totals
    }.fold(
        { e -> throw AssertionError(e.toString()) },
        { it },
    )
