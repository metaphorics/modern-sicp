// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.62

package sicp.ch4.solutions

import sicp.ch4.QAnd
import sicp.ch4.QList
import sicp.ch4.QPattern
import sicp.ch4.QRule
import sicp.ch4.QTerm
import sicp.ch4.QVar
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver
import sicp.ch4.reify
import sicp.ch4.renderTerm

// Exercise 4.62: the last-pair rule. The rules state the recursion over
// list structure: a one-element list is its own last pair, and the last
// pair of a longer list is the last pair of its tail. The backward query
// (last-pair ?x (3)) has infinitely many answers -- any list ending in 3
// -- so the probe reports the shape of the first three answers.

/** The last-pair rules of the exercise. */
internal fun addLastPair(db: QueryDatabase) {
    db.addRule(QRule(list(sym("last-pair"), list(v("x")), list(v("x"))), QAnd(emptyList())))
    db.addRule(
        QRule(
            list(sym("last-pair"), improper(listOf(v("u")), v("v")), v("y")),
            QPattern(list(sym("last-pair"), v("v"), v("y"))),
        ),
    )
}

/** The last-pair queries with their answers; the backward query reports
 * the shape of its first three answers. The backward answers carry
 * unbound rule variables whose freshened names are engine detail, so the
 * probe pins the stable shape instead: each answer's length and final
 * element, which grow one per answer. The backward generation is
 * unbounded, so it runs under the loop detector at depth 8, which keeps
 * exactly the answers the bound admits. */
public fun lastPairQueries(): List<String> {
    val db = QueryDatabase()
    addLastPair(db)
    val driver = QueryDriver.streaming(db)
    val x = listOf(v("x"))
    val one = answerLines(driver, QPattern(list(sym("last-pair"), list(sym("3")), v("x"))), x)
    val three = answerLines(driver, QPattern(list(sym("last-pair"), list(sym("1"), sym("2"), sym("3")), v("x"))), x)
    val unknown = answerLines(driver, QPattern(list(sym("last-pair"), list(sym("2"), v("x")), list(sym("3")))), x)
    val bounded = QueryDriver.loopDetecting(db, 8)
    val backward =
        bounded
            .run(QPattern(list(sym("last-pair"), v("x"), list(sym("3")))), x)
            .take(3)
            .mapIndexed { index, frame -> backwardShape(index, reify(v("x"), frame)) }
            .toList()
    return one + three + unknown + backward
}

/** The stable shape of one instantiated backward answer: its length and
 * final element. */
internal fun backwardShape(
    index: Int,
    term: QTerm,
): String {
    val items = (term as? QList)?.items ?: emptyList()
    return "backward answer " + index + ": x has " + items.size + " item(s), ends " + renderTerm(items.last())
}
