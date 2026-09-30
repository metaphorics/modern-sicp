// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.74: the simple flatmap order.

package sicp.ch4.solutions

import sicp.ch4.QAnd
import sicp.ch4.QPattern
import sicp.ch4.QueryDriver

// Exercise 4.74: Alyssa's simple flatmap -- map, then flatten, with no
// interleaving across the outer frames. The grounded outer frame owns
// an unbounded inner stream, so the prefix comes off its front: the
// first answer pairs the grounded outer frame with the inner stream's
// first answer, and the rest of the infinite inner stream waits behind
// it. The cycle runs under the loop detector, which bounds its
// re-entry depth so the prefix terminates either way. On finite data
// both flatmaps agree; the prefix shows where they part.

// Exercise 4.74: the grounded outer frame's infinite inner stream fills the prefix.

/** The simple flatmap prefix over the loves cycle. */
public fun simpleFlatmapDemo(): List<String> {
    val db = microshaftSystem()
    addLoves(db)
    val driver = QueryDriver.loopDetecting(db, 8)
    val query =
        QAnd(
            listOf(
                QPattern(list(sym("loves"), list(sym("Minnie"), sym("Mouse")), list(sym("Mickey"), sym("Mouse")))),
                QPattern(list(sym("loves"), list(sym("Mickey"), sym("Mouse")), v("y"))),
            ),
        )
    return takeAnswerLines(driver, query, listOf(v("y")), 1)
}
