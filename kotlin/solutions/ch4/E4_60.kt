// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.60: the doubled lives-near listing and the
// salary-ordered unique variant.

package sicp.ch4.solutions

import sicp.ch4.QuerySystem

private const val LIVES_NEAR_UNIQUE =
    """
    (assert! (rule (lives-near-unique ?person-1 ?person-2)
                   (and (lives-near ?person-1 ?person-2)
                        (address ?person-1 (?town-1 . ?rest-1))
                        (address ?person-2 (?town-2 . ?rest-2))
                        (salary ?person-1 ?salary-1)
                        (salary ?person-2 ?salary-2)
                        (lisp-value < ?salary-1 ?salary-2))))
    """

public fun systemWithLivesNearUnique(): QuerySystem {
    val system = microshaftSystem()
    system.load(LIVES_NEAR_UNIQUE)
    return system
}

/** The single-name query finds the ride shares; the full query lists
 * every pair twice, once per binding order of the two address conjuncts;
 * the unique variant keeps one order per pair, keyed on the salary
 * inequality, which holds for every Microshaft pair. */
public fun livesNearQueries(system: QuerySystem): List<String> {
    val out = mutableListOf<String>()
    for (
    query in
    listOf(
        "(lives-near ?person (Hacker Alyssa P))",
        "(lives-near ?person-1 ?person-2)",
        "(lives-near-unique ?person-1 ?person-2)",
    )
    ) {
        out.add("query: $query")
        out.addAll(answersOf(system, query))
    }
    return out
}

public fun livesNearQueries(): List<String> = livesNearQueries(systemWithLivesNearUnique())
