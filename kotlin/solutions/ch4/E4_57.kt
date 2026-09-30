// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.57

package sicp.ch4.solutions

import sicp.ch4.QAnd
import sicp.ch4.QGuard
import sicp.ch4.QNot
import sicp.ch4.QOr
import sicp.ch4.QPattern
import sicp.ch4.QRule
import sicp.ch4.QVar
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver

// Exercise 4.57: the can-replace rule. A person can replace another when
// they hold the same job or one that can do it, and they are not the same
// person. The second query joins the rule with the salary facts and a
// guard to find replacements who cost less.

/** The can-replace rule of the exercise. */
internal fun addCanReplace(db: QueryDatabase) {
    db.addRule(
        QRule(
            list(sym("can-replace"), v("person-1"), v("person-2")),
            QAnd(
                listOf(
                    QPattern(list(sym("job"), v("person-1"), v("job-1"))),
                    QPattern(list(sym("job"), v("person-2"), v("job-2"))),
                    QOr(
                        listOf(
                            QPattern(list(sym("same"), v("job-1"), v("job-2"))),
                            QPattern(list(sym("can-do-job"), v("job-1"), v("job-2"))),
                        ),
                    ),
                    QNot(QPattern(list(sym("same"), v("person-1"), v("person-2")))),
                ),
            ),
        ),
    )
}

/** The can-replace queries with their answers. */
public fun canReplaceQueries(): List<String> {
    val db = microshaftSystem()
    addCanReplace(db)
    val driver = QueryDriver.streaming(db)
    val replacement =
        answerLines(
            driver,
            QPattern(list(sym("can-replace"), v("x"), list(sym("Fect"), sym("Cy"), sym("D")))),
            listOf(v("x")),
        )
    val cheaper =
        answerLines(
            driver,
            QAnd(
                listOf(
                    QPattern(list(sym("can-replace"), v("person-1"), v("person-2"))),
                    QPattern(list(sym("salary"), v("person-1"), v("salary-1"))),
                    QPattern(list(sym("salary"), v("person-2"), v("salary-2"))),
                    QGuard({ terms -> termLong(terms[0]) < termLong(terms[1]) }, listOf(v("salary-1"), v("salary-2"))),
                ),
            ),
            listOf(v("person-1"), v("person-2")),
        )
    return replacement + cheaper
}
