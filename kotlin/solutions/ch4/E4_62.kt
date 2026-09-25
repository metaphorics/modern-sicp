// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.62: the last-pair rules, including the
// divergent unanchored query.

package sicp.ch4.solutions

import sicp.ch4.QuerySystem

private const val LAST_PAIR_RULES =
    """
    (assert! (rule (last-pair (?x) (?x))))
    (assert! (rule (last-pair (?u . ?v) ?y)
                   (last-pair ?v ?y)))
    """

public fun systemWithLastPair(): QuerySystem {
    val system = QuerySystem()
    system.load(LAST_PAIR_RULES)
    return system
}

/** The anchored queries terminate; (last-pair ?x (3)) generates one
 * longer prefix per step-rule application, and each candidate's tail is
 * a fresh renamed variable, so the answers show ?v-ids. The demo pins
 * the first three answers of the divergent query and reports the rest. */
public fun lastPairQueries(): List<String> {
    val system = systemWithLastPair()
    val out = mutableListOf<String>()
    for (
    query in
    listOf(
        "(last-pair (3) ?x)",
        "(last-pair (1 2 3) ?x)",
        "(last-pair (2 ?x) (3))",
    )
    ) {
        out.add("query: $query")
        out.addAll(answersOf(system, query))
    }
    out.add("query: (last-pair ?x (3)) -- divergent; first three answers:")
    out.addAll(answersUpto(system, "(last-pair ?x (3))", 3))
    return out
}
