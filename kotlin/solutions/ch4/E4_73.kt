// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.73: the delay in flattening.

package sicp.ch4.solutions

import sicp.ch4.QAnd
import sicp.ch4.QOr
import sicp.ch4.QPattern
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver

// Exercise 4.73: why flattening must be delayed. The second disjunct,
// the married cycle, answers forever under the streaming driver. An
// undelayed flatten would force that whole branch before delivering
// anything and never return; the engine flattens lazy sequences, so a
// prefix arrives on demand. The disjunction interleaves the branches:
// the finite conjunction -- everyone Ben supervises, in data-base order
// -- alternates with Minnie from the cycle, and six answers exhaust the
// finite branch while the infinite one is still unexplored beyond them.
// Only the shared variable is asked, so every frame answers it: no
// branch leaves a variable unbound.

// Exercise 4.73: a demand-driven prefix of a disjunction with an infinite branch.

/** Finite conjunction first, infinite cycle after, one shared variable. */
public fun flattenDelayDebate(): List<String> {
    val db = microshaftSystem()
    addMarriedLoop(db)
    val driver = QueryDriver.streaming(db)
    val query =
        QOr(
            listOf(
                QAnd(
                    listOf(
                        QPattern(list(sym("supervisor"), v("who"), list(sym("Bitdiddle"), sym("Ben")))),
                        QPattern(list(sym("job"), v("who"), v("job"))),
                    ),
                ),
                QPattern(list(sym("married"), sym("Mickey"), v("who"))),
            ),
        )
    return takeAnswerLines(driver, query, listOf(v("who")), 6)
}
