// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.63

package sicp.ch4.solutions

import sicp.ch4.QAnd
import sicp.ch4.QFact
import sicp.ch4.QPattern
import sicp.ch4.QRule
import sicp.ch4.QVar
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver

// Exercise 4.63: family relations. The Genesis data base is a list of son
// facts; the grandson rule joins two of them. The three queries walk the
// relation forward and backward.

/** The Genesis data base and the grandson rule. */
internal fun genesisSystem(): QueryDatabase {
    val db = QueryDatabase()
    db.assertFact(QFact(list(sym("son"), sym("Adam"), sym("Cain"))))
    db.assertFact(QFact(list(sym("son"), sym("Cain"), sym("Enoch"))))
    db.assertFact(QFact(list(sym("son"), sym("Enoch"), sym("Irad"))))
    db.assertFact(QFact(list(sym("son"), sym("Irad"), sym("Mehujael"))))
    db.assertFact(QFact(list(sym("son"), sym("Mehujael"), sym("Methushael"))))
    db.assertFact(QFact(list(sym("son"), sym("Methushael"), sym("Lamech"))))
    db.assertFact(QFact(list(sym("son"), sym("Lamech"), sym("Jabal"))))
    db.assertFact(QFact(list(sym("son"), sym("Lamech"), sym("Jubal"))))
    db.addRule(
        QRule(
            list(sym("grandson"), v("s"), v("g")),
            QAnd(
                listOf(
                    QPattern(list(sym("son"), v("s"), v("son"))),
                    QPattern(list(sym("son"), v("son"), v("g"))),
                ),
            ),
        ),
    )
    return db
}

/** The Genesis queries with their answers. */
public fun genesisQueries(): List<String> {
    val driver = QueryDriver.streaming(genesisSystem())
    val x = listOf(v("x"))
    val cain = answerLines(driver, QPattern(list(sym("grandson"), sym("Cain"), v("x"))), x)
    val lamech = answerLines(driver, QPattern(list(sym("son"), sym("Lamech"), v("x"))), x)
    val methushael = answerLines(driver, QPattern(list(sym("grandson"), v("x"), sym("Methushael"))), x)
    return cain + lamech + methushael
}
