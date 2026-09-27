// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.58: the big-shot rule.

package sicp.ch4.solutions

import sicp.ch4.QuerySystem

private const val BIG_SHOT_RULE =
    """
    (assert! (rule (big-shot ?person ?division)
                   (and (job ?person (?division . ?rest))
                        (or (not (supervisor ?person ?supervisor))
                            (and (supervisor ?person ?supervisor)
                                 (not (job ?supervisor (?division . ?rest-2))))))))
    """

public fun systemWithBigShot(): QuerySystem {
    val system = microshaftSystem()
    system.load(BIG_SHOT_RULE)
    return system
}

/** A person is a big shot in a division when they work there and their
 * supervisor -- if they have one -- works in a different division.
 * Warbucks qualifies through the no-supervisor branch of the or. */
public fun bigShotQuery(system: QuerySystem): List<String> {
    val out = mutableListOf<String>()
    out.add("query: (big-shot ?person ?division)")
    out.addAll(answersOf(system, "(big-shot ?person ?division)"))
    return out
}

public fun bigShotQuery(): List<String> = bigShotQuery(systemWithBigShot())
