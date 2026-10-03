// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.58

package sicp.ch4.solutions

import sicp.ch4.QAnd
import sicp.ch4.QNot
import sicp.ch4.QOr
import sicp.ch4.QPattern
import sicp.ch4.QRule
import sicp.ch4.QVar
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver

// Exercise 4.58: the big-shot rule. A person is a big shot of their
// division when they have no supervisor at all, or their supervisor sits
// in another division.

/** The big-shot rule of the exercise. */
internal fun addBigShot(db: QueryDatabase) {
    db.addRule(
        QRule(
            list(sym("big-shot"), v("person"), v("division")),
            QAnd(
                listOf(
                    QPattern(list(sym("job"), v("person"), improper(listOf(v("division")), v("rest")))),
                    QOr(
                        listOf(
                            QNot(QPattern(list(sym("supervisor"), v("person"), v("boss")))),
                            QAnd(
                                listOf(
                                    QPattern(list(sym("supervisor"), v("person"), v("boss"))),
                                    QNot(QPattern(list(sym("job"), v("boss"), improper(listOf(v("division")), v("other"))))),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}

/** The big-shot query with its answers.
 * => administration: (Warbucks Oliver); computer: (Bitdiddle Ben);
 * accounting: (Scrooge Eben) */
public fun bigShotQuery(): List<String> {
    val db = microshaftSystem()
    addBigShot(db)
    return answerLines(
        QueryDriver.streaming(db),
        QPattern(list(sym("big-shot"), v("person"), v("division"))),
        listOf(v("person"), v("division")),
    )
}
