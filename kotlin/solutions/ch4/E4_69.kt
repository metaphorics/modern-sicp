// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.69: the greats chain over the Genesis data base.

package sicp.ch4.solutions

import sicp.ch4.QuerySystem

private const val GREATS =
    """
    (assert! (rule ((grandson) ?x ?y)
                   (grandson ?x ?y)))
    (assert! (rule ((great . ?relationship) ?x ?y)
                   (and (?relationship ?x ?z)
                        (son ?z ?y))))
    """

public fun systemWithGreats(): QuerySystem {
    val system = genesisSystem()
    system.load(GREATS)
    return system
}

/** One great per recursion: (great . rel) x y holds when rel x z and z
 * is a son of y, so the trailing grandson contributes the last two
 * links. The unanchored (?relationship Adam Irad) query generates
 * relationship lists of every depth and diverges in this engine; the
 * segment records that honestly while the anchored queries pin the
 * chain. */
public fun greatsQueries(): List<String> {
    val system = systemWithGreats()
    val out = mutableListOf<String>()

    fun segment(
        label: String,
        force: () -> List<String>,
    ) {
        out.add(label)
        out.addAll(
            try {
                force()
            } catch (overflow: StackOverflowError) {
                listOf("the unanchored relationship generator diverges in this engine before the first answer")
            } catch (fault: OutOfMemoryError) {
                listOf("the unanchored relationship generator exhausts the heap before the first answer")
            },
        )
    }

    segment("query: ((great grandson) ?g ?ggs)") { answersOf(system, "((great grandson) ?g ?ggs)") }
    segment("query: (?relationship Adam Irad) -- first answer:") { answersUpto(system, "(?relationship Adam Irad)", 1) }
    segment("query: ((great great great great great grandson) Adam ?d)") {
        answersOf(system, "((great great great great great grandson) Adam ?d)")
    }
    return out
}
