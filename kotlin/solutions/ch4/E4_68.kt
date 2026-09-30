// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.68: the reverse rules over append-to-form.

package sicp.ch4.solutions

import sicp.ch4.QAnd
import sicp.ch4.QPattern
import sicp.ch4.QRule
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver

// Exercise 4.68: reverse as rules over the section's append. The base
// rule reverses the empty list; the step rule reverses the tail and
// appends the head behind it. Forward queries terminate with one
// answer each. The backward query generates candidate lists forever
// under the streaming driver, so it runs under the loop detector at
// depth 8, which bounds the generation and keeps the one answer the
// depth admits.

// Exercise 4.68: reverse by rule; the backward query runs bounded.

/** The book's reverse rules as query data. */
internal fun addReverseRules(db: QueryDatabase) {
    db.addRule(QRule(list(sym("reverse"), list(), list()), QAnd(emptyList())))
    db.addRule(
        QRule(
            list(sym("reverse"), improper(listOf(v("u")), v("v")), v("y")),
            QAnd(
                listOf(
                    QPattern(list(sym("reverse"), v("v"), v("z"))),
                    QPattern(list(sym("append-to-form"), v("z"), list(v("u")), v("y"))),
                ),
            ),
        ),
    )
}

/** Forward reversals answer once each; the backward query answers once
 * under the depth bound. */
public fun reverseQueries(): List<String> {
    val db = microshaftSystem()
    addReverseRules(db)
    val forward = QueryDriver.streaming(db)
    val x = listOf(v("x"))
    val oneTwoThree = answerLines(forward, QPattern(list(sym("reverse"), list(sym("1"), sym("2"), sym("3")), v("x"))), x)
    val abcd =
        answerLines(
            forward,
            QPattern(list(sym("reverse"), list(sym("a"), sym("b"), sym("c"), sym("d")), v("x"))),
            x,
        )
    val bounded = QueryDriver.loopDetecting(db, 8)
    val backward =
        answerLines(bounded, QPattern(list(sym("reverse"), v("x"), list(sym("1"), sym("2"), sym("3")))), x)
    return oneTwoThree + abcd + backward
}
