// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.55: simple queries over the Microshaft data base.

package sicp.ch4.solutions

import sicp.ch4.QuerySystem

/** The three simple queries with their answers, in data-base order. The
 * strict matcher of 4.4.4.3 matches only the two-element accounting job
 * in (b); the dotted tail of (c) reaches the whole rest of the address. */
public fun simpleQueries(system: QuerySystem): List<String> {
    val out = mutableListOf<String>()
    for (
    query in
    listOf(
        "(supervisor ?name (Bitdiddle Ben))",
        "(job ?name (accounting . ?title))",
        "(address ?name (Slumerville . ?where))",
    )
    ) {
        out.add("query: $query")
        out.addAll(answersOf(system, query))
    }
    return out
}

public fun simpleQueries(): List<String> = simpleQueries(microshaftSystem())
