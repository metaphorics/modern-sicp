// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.69: the greats chain over the Genesis data base.

package sicp.ch4.solutions

import sicp.ch4.QAnd
import sicp.ch4.QPattern
import sicp.ch4.QRule
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver

// Exercise 4.69: relationships as data. One `great` per recursion level:
// `(great . relationship)` holds of `x` and `y` when the shorter
// relationship holds of `x` and some `z` a son-link above `y`, and the
// bare grandson ends the chain. The relationship variable in the body
// sits in head position, so without a guard it also matches the
// Genesis `grandson` rule and the `son` facts themselves, admitting the
// improper relationships `[great | grandson]` and `[great, great | son]`.
// The `ends-in-grandson` guard restricts the tail to a proper list
// ending in `grandson`, as the exercise intends. The anchored queries
// pin the chain; the unanchored relationship query generates candidate
// tails of every length, so it runs under the loop detector, which
// bounds the generation at depth 8 and keeps the answers the bound
// admits.

/** The greats rules of the exercise, over the Genesis sons. */
internal fun addGreats(db: QueryDatabase) {
    db.addRule(QRule(list(sym("ends-in-grandson"), list(sym("grandson"))), QAnd(emptyList())))
    db.addRule(
        QRule(
            list(sym("ends-in-grandson"), improper(listOf(v("head")), v("rest"))),
            QPattern(list(sym("ends-in-grandson"), v("rest"))),
        ),
    )
    db.addRule(
        QRule(
            list(list(sym("grandson")), v("x"), v("y")),
            QPattern(list(sym("grandson"), v("x"), v("y"))),
        ),
    )
    db.addRule(
        QRule(
            list(improper(listOf(sym("great")), v("relationship")), v("x"), v("y")),
            QAnd(
                listOf(
                    QPattern(list(sym("ends-in-grandson"), v("relationship"))),
                    QPattern(list(v("relationship"), v("x"), v("z"))),
                    QPattern(list(sym("son"), v("z"), v("y"))),
                ),
            ),
        ),
    )
}

/** The anchored chains, then the bounded open relationship query. */
public fun greatsQueries(): List<String> {
    val db = genesisSystem()
    addGreats(db)
    val forward = QueryDriver.streaming(db)
    val g = listOf(v("g"), v("ggs"))
    val greatGrandsons =
        answerLines(
            forward,
            QPattern(list(list(sym("great"), sym("grandson")), v("g"), v("ggs"))),
            g,
        )
    val fifth =
        answerLines(
            forward,
            QPattern(
                list(
                    list(sym("great"), sym("great"), sym("great"), sym("great"), sym("great"), sym("grandson")),
                    sym("Adam"),
                    v("d"),
                ),
            ),
            listOf(v("d")),
        )
    val bounded = QueryDriver.loopDetecting(db, 8)
    val open =
        answerLines(bounded, QPattern(list(v("relationship"), sym("Adam"), sym("Irad"))), listOf(v("relationship")))
    return greatGrandsons + fifth + open
}
