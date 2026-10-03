// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.61

package sicp.ch4.solutions

import sicp.ch4.QAnd
import sicp.ch4.QPattern
import sicp.ch4.QRule
import sicp.ch4.QVar
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver

// Exercise 4.61: the next-to rule. The rule reads "x is next to y in a
// list" off the list structure itself: the first clause matches a list
// whose first two elements are x and y, the second clause walks the tail.
// Both queries of the exercise run the rule over data lists.

/** The next-to rules of the exercise. */
internal fun addNextTo(db: QueryDatabase) {
    db.addRule(
        QRule(
            list(v("x"), sym("next-to"), v("y"), sym("in"), improper(listOf(v("x"), v("y")), v("z"))),
            QAnd(emptyList()),
        ),
    )
    db.addRule(
        QRule(
            list(v("x"), sym("next-to"), v("y"), sym("in"), improper(listOf(v("w")), v("z"))),
            QPattern(list(v("x"), sym("next-to"), v("y"), sym("in"), v("z"))),
        ),
    )
}

/** The next-to queries with their answers. */
public fun nextToQueries(): List<String> {
    val db = QueryDatabase()
    addNextTo(db)
    val driver = QueryDriver.streaming(db)
    val first =
        answerLines(
            driver,
            QPattern(
                list(
                    v("x"),
                    sym("next-to"),
                    v("y"),
                    sym("in"),
                    list(sym("1"), list(sym("2"), sym("3")), sym("4")),
                ),
            ),
            listOf(v("x"), v("y")),
        )
    val second =
        answerLines(
            driver,
            QPattern(
                list(
                    v("x"),
                    sym("next-to"),
                    sym("1"),
                    sym("in"),
                    list(sym("2"), sym("1"), sym("3"), sym("1")),
                ),
            ),
            listOf(v("x")),
        )
    return first + second
}
