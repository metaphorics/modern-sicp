// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.56

package sicp.ch4.solutions

import sicp.ch4.QAnd
import sicp.ch4.QGuard
import sicp.ch4.QNot
import sicp.ch4.QPattern
import sicp.ch4.QVar
import sicp.ch4.QueryDriver

// Exercise 4.56: compound queries. The conjunction retrieves one frame per
// match, the guard runs its host predicate over bound terms, and the
// negation filters frames the subquery cannot extend.

/** The three compound queries with their answers. */
public fun compoundQueries(): List<String> {
    val db = microshaftSystem()
    val driver = QueryDriver.streaming(db)
    val person = listOf(v("person"))
    val personWhere = listOf(v("person"), v("where"))
    val salary = listOf(v("person"), v("amount"))
    val underBen =
        answerLines(
            driver,
            QAnd(
                listOf(
                    QPattern(list(sym("supervisor"), v("person"), list(sym("Bitdiddle"), sym("Ben")))),
                    QPattern(list(sym("address"), v("person"), v("where"))),
                ),
            ),
            personWhere,
        )
    val belowBen =
        answerLines(
            driver,
            QAnd(
                listOf(
                    QPattern(list(sym("salary"), v("person"), v("amount"))),
                    QPattern(list(sym("salary"), list(sym("Bitdiddle"), sym("Ben")), v("ben-amount"))),
                    QGuard({ terms -> termLong(terms[0]) < termLong(terms[1]) }, listOf(v("amount"), v("ben-amount"))),
                ),
            ),
            salary,
        )
    val notComputerBoss =
        answerLines(
            driver,
            QAnd(
                listOf(
                    QPattern(list(sym("supervisor"), v("person"), v("boss"))),
                    QNot(QPattern(list(sym("job"), v("boss"), improper(listOf(sym("computer")), v("type"))))),
                ),
            ),
            person,
        )
    return underBen + belowBen + notComputerBoss
}
