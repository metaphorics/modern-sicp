// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.60

package sicp.ch4.solutions

import sicp.ch4.QAnd
import sicp.ch4.QGuard
import sicp.ch4.QNot
import sicp.ch4.QPattern
import sicp.ch4.QRule
import sicp.ch4.QVar
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver

// Exercise 4.60: lives-near duplicates. The prose rule answers each pair
// twice, once per orientation. The unique variant keeps the pair whose
// first person appears later in the data base, so each pair survives once
// and the answers drop from eight to four.

/** The data-base order of the Microshaft people. */
internal val microshaftPeople: List<String> =
    listOf(
        "Bitdiddle Ben",
        "Hacker Alyssa P",
        "Fect Cy D",
        "Tweakit Lem E",
        "Reasoner Louis",
        "Warbucks Oliver",
        "Scrooge Eben",
        "Cratchet Robert",
        "Aull DeWitt",
    )

/** The unique variant of the rule: one line per unordered pair. */
internal fun addLivesNearUnique(db: QueryDatabase) {
    db.addRule(
        QRule(
            list(sym("lives-near-unique"), v("person-1"), v("person-2")),
            QAnd(
                listOf(
                    QPattern(list(sym("address"), v("person-1"), improper(listOf(v("town")), v("rest-1")))),
                    QPattern(list(sym("address"), v("person-2"), improper(listOf(v("town")), v("rest-2")))),
                    QNot(QPattern(list(sym("same"), v("person-1"), v("person-2")))),
                    QGuard(
                        { terms ->
                            microshaftPeople.indexOf(personKey(terms[0])) > microshaftPeople.indexOf(personKey(terms[1]))
                        },
                        listOf(v("person-1"), v("person-2")),
                    ),
                ),
            ),
        ),
    )
}

/** The lives-near queries: both orientations, then the unique variant. */
public fun livesNearQueries(): List<String> {
    val db = microshaftSystem()
    addLivesNearUnique(db)
    val driver = QueryDriver.streaming(db)
    val nearAlyssa =
        answerLines(
            driver,
            QPattern(list(sym("lives-near"), v("person"), list(sym("Hacker"), sym("Alyssa"), sym("P")))),
            listOf(v("person")),
        )
    val pairs = answerLines(driver, QPattern(list(sym("lives-near"), v("person-1"), v("person-2"))), listOf(v("person-1"), v("person-2")))
    val unique =
        answerLines(
            driver,
            QPattern(list(sym("lives-near-unique"), v("person-1"), v("person-2"))),
            listOf(v("person-1"), v("person-2")),
        )
    return nearAlyssa + pairs + unique
}
