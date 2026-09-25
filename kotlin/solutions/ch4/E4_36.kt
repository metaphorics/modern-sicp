// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.36

package sicp.ch4.solutions

import sicp.ch4.AmbEvaluator

/** The fair generator's first six triples: the hypotenuse runs
 * unbounded and each finite k is searched out fully.
 * => [(3 4 5), (6 8 10), (5 12 13), (9 12 15), (8 15 17), (12 16 20)] */
public fun fairTriplesFirstSix(): List<String> =
    answerLinesFaulted(
        ::AmbEvaluator,
        "$AMB_BASE_PRELUDE\n$FAIR_TRIPLE_PROGRAM",
        "(a-pythagorean-triple)",
        limit = 6,
    )

/** The naive replacement never leaves its first two choices: the
 * innermost unbounded choice never exhausts, so no triple ever arrives
 * before the 600-choice budget raises its typed fault.
 * => "choice budget exhausted after 600 choices" */
public fun naiveBudgetFault(): String = budgetedFault("$AMB_BASE_PRELUDE\n$NAIVE_TRIPLE_PROGRAM", "(a-pythagorean-triple-naive)", cap = 600)
