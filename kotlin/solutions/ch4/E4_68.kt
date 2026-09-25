// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.68: the reverse rules over append-to-form.

package sicp.ch4.solutions

import sicp.ch4.QuerySystem

private const val REVERSE_RULES =
    """
    (assert! (rule (reverse () ())))
    (assert! (rule (reverse (?u . ?v) ?y)
                   (and (reverse ?v ?z)
                        (append-to-form ?z (?u) ?y))))
    """

public fun systemWithReverse(): QuerySystem {
    val system = microshaftSystem()
    system.load(REVERSE_RULES)
    return system
}

/** Forward runs terminate; the backward query (reverse ?x (1 2 3)) has
 * exactly one answer but the engine generates candidate lists forever
 * before finding it, so the demo pins its first answer only. */
public fun reverseQueries(): List<String> {
    val system = systemWithReverse()
    val out = mutableListOf<String>()
    out.add("query: (reverse (1 2 3) ?x)")
    out.addAll(answersOf(system, "(reverse (1 2 3) ?x)"))
    out.add("query: (reverse (a b c d) ?x)")
    out.addAll(answersOf(system, "(reverse (a b c d) ?x)"))
    out.add("query: (reverse ?x (1 2 3)) -- first answer:")
    val backward =
        try {
            answersUpto(system, "(reverse ?x (1 2 3))", 1)
        } catch (overflow: StackOverflowError) {
            listOf("the unanchored generation diverges in this engine before the first answer survives the append filter")
        } catch (fault: OutOfMemoryError) {
            listOf("the unanchored generation exhausts the heap before the first answer survives the append filter")
        }
    out.addAll(backward)
    return out
}
