// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.64: Louis Reasoner's swapped outranked-by, whose
// recursion precedes the supervisor test and diverges.

package sicp.ch4.solutions

import sicp.ch4.QAnd
import sicp.ch4.QOr
import sicp.ch4.QPattern
import sicp.ch4.QRule
import sicp.ch4.QueryDriver

// Exercise 4.64: Louis's swapped conjuncts as rule data. The book's rule
// tests the supervisor link before recursing, so its stream completes;
// Louis recurses first, so the anchored query still finds its supervisor
// answer but the stream behind it never ends. The loop-detecting driver
// bounds the re-entry depth, and the same query completes under it.

// Exercise 4.64: recursion-first answers once, then diverges; the detector bounds it.

/** Louis's rule: the recursive conjunct runs before the supervisor test. */
internal fun louisOutrankedRule(): QRule =
    QRule(
        list(sym("outranked-by"), v("staff-person"), v("boss")),
        QOr(
            listOf(
                QPattern(list(sym("supervisor"), v("staff-person"), v("boss"))),
                QAnd(
                    listOf(
                        QPattern(list(sym("outranked-by"), v("middle-manager"), v("boss"))),
                        QPattern(list(sym("supervisor"), v("staff-person"), v("middle-manager"))),
                    ),
                ),
            ),
        ),
    )

/** The anchored query both rules answer. */
internal fun anchoredOutranked(): QPattern =
    QPattern(list(sym("outranked-by"), list(sym("Bitdiddle"), sym("Ben")), list(sym("Warbucks"), sym("Oliver"))))

/** The anchored answer arrives under Louis, the book completes, and the
 * detector bounds Louis's stream. */
public fun louisOutranked(): List<String> {
    val louisDb = microshaftDatabase()
    louisDb.addRule(louisOutrankedRule())
    // Louis's stream diverges past its first answer, so both Louis runs
    // go through the loop detector, which bounds the re-entry depth.
    val louis = QueryDriver.loopDetecting(louisDb, 8)
    val first =
        louis
            .run(anchoredOutranked(), emptyList())
            .take(1)
            .toList()
            .size
    val stock = microshaftSystem()
    val book =
        QueryDriver
            .streaming(stock)
            .run(anchoredOutranked(), emptyList())
            .take(100)
            .toList()
            .size
    // Returning from this call is the probe: an unbounded Louis stream
    // would never finish the take.
    louis.run(anchoredOutranked(), emptyList()).take(100).toList()
    return listOf(
        "louis anchored: first answer arrives ($first frame)",
        "book order: completes with $book frame",
        "loop detector bounds louis: stream completes",
    )
}
