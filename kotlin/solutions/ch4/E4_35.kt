// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.35

package sicp.ch4.solutions

import arrow.core.raise.either
import sicp.ch4.AmbEvaluator

/** The triples between 1 and 20, in the search's own order, then
 * exhaustion. => [(3 4 5), (5 12 13), (6 8 10), (8 15 17), (9 12 15),
 * (12 16 20)] */
public fun triplesBetween20(): List<String> =
    either {
        answerLines(
            newDriver(::AmbEvaluator, "$AMB_BASE_PRELUDE\n$TRIPLES_PROGRAM"),
            "(a-pythagorean-triple-between 1 20)",
        )
    }.fold(
        { e -> throw AssertionError(e.toString()) },
        { it },
    )
