// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.57: the can-replace rule.

package sicp.ch4.solutions

import sicp.ch4.QuerySystem

private const val CAN_REPLACE_RULE =
    """
    (assert! (rule (can-replace ?person-1 ?person-2)
                   (and (job ?person-1 ?job-1)
                        (job ?person-2 ?job-2)
                        (or (can-do-job ?job-1 ?job-2)
                            (same ?job-1 ?job-2))
                        (not (same ?person-1 ?person-2)))))
    """

public fun systemWithCanReplace(): QuerySystem {
    val system = microshaftSystem()
    system.load(CAN_REPLACE_RULE)
    return system
}

/** (a) everyone who can replace Cy D. Fect: Hacker by doing the same
 * job, Ben because the wizard can do the programmer's job; (b) the
 * replacements where the replacee earns more, with both salaries. */
public fun canReplaceQueries(system: QuerySystem): List<String> {
    val out = mutableListOf<String>()
    for (
    query in
    listOf(
        "(can-replace ?x (Fect Cy D))",
        "(and (can-replace ?person-1 ?person-2)" +
            " (salary ?person-1 ?salary-1) (salary ?person-2 ?salary-2)" +
            " (lisp-value < ?salary-1 ?salary-2))",
    )
    ) {
        out.add("query: $query")
        out.addAll(answersOf(system, query))
    }
    return out
}

public fun canReplaceQueries(): List<String> = canReplaceQueries(systemWithCanReplace())
