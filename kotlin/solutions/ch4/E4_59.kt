// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.59: meeting assertions and Alyssa's meeting-time
// rule.

package sicp.ch4.solutions

import sicp.ch4.QuerySystem

private const val MEETINGS =
    """
    (assert! (meeting accounting (Monday 9am)))
    (assert! (meeting administration (Monday 10am)))
    (assert! (meeting computer (Wednesday 3pm)))
    (assert! (meeting administration (Friday 1pm)))
    (assert! (meeting whole-company (Wednesday 4pm)))
    (assert! (rule (meeting-time ?person ?day-and-time)
                   (or (meeting whole-company ?day-and-time)
                       (and (meeting ?division ?day-and-time)
                            (job ?person (?division . ?rest))))))
    """

public fun systemWithMeetings(): QuerySystem {
    val system = microshaftSystem()
    system.load(MEETINGS)
    return system
}

/** (a) Friday's meetings through the data base; (b) Alyssa's Wednesday
 * meetings through the rule: the whole-company disjunct interleaves
 * first, so the 4pm company meeting lists before the 3pm computer one. */
public fun meetingQueries(system: QuerySystem): List<String> {
    val out = mutableListOf<String>()
    for (
    query in
    listOf(
        "(meeting ?division (Friday ?time))",
        "(meeting-time (Hacker Alyssa P) (Wednesday ?time))",
        "(meeting-time (Hacker Alyssa P) (Friday ?time))",
    )
    ) {
        out.add("query: $query")
        out.addAll(answersOf(system, query))
    }
    return out
}

public fun meetingQueries(): List<String> = meetingQueries(systemWithMeetings())
