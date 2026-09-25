// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.61: the next-to rules and the book's two
// queries.

package sicp.ch4.solutions

import sicp.ch4.QuerySystem

private const val NEXT_TO_RULES =
    """
    (assert! (rule (?x next-to ?y in (?x ?y . ?u))))
    (assert! (rule (?x next-to ?y in (?v . ?z))
                   (?x next-to ?y in ?z)))
    """

public fun systemWithNextTo(): QuerySystem {
    val system = QuerySystem()
    system.load(NEXT_TO_RULES)
    return system
}

/** The base rule answers an adjacency outright; the step rule recurses
 * on the tail, so in insertion order the outermost adjacency answers
 * first and the innermost one lands last. */
public fun nextToQueries(system: QuerySystem): List<String> {
    val out = mutableListOf<String>()
    for (
    query in
    listOf(
        "(?x next-to ?y in (1 (2 3) 4))",
        "(?x next-to 1 in (2 1 3 1))",
    )
    ) {
        out.add("query: $query")
        out.addAll(answersOf(system, query))
    }
    return out
}

public fun nextToQueries(): List<String> = nextToQueries(systemWithNextTo())
