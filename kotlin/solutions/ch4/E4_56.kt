// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.56: compound queries over the Microshaft data base.

package sicp.ch4.solutions

import sicp.ch4.QuerySystem

/** (a) Ben's supervisees with their addresses; (b) the lower-paid with
 * their salaries; (c) the supervisees whose supervisor works outside the
 * computer division, reached with a dotted-tail not-filter. */
public fun compoundQueries(system: QuerySystem): List<String> {
    val out = mutableListOf<String>()
    for (
    query in
    listOf(
        "(and (supervisor ?person (Bitdiddle Ben)) (address ?person ?where))",
        "(and (salary ?person ?amount) (salary (Bitdiddle Ben) ?ben-amount)" +
            " (lisp-value < ?amount ?ben-amount))",
        "(and (supervisor ?person ?supervisor) (job ?supervisor ?job)" +
            " (not (job ?supervisor (computer . ?type))))",
    )
    ) {
        out.add("query: $query")
        out.addAll(answersOf(system, query))
    }
    return out
}

public fun compoundQueries(): List<String> = compoundQueries(microshaftSystem())
