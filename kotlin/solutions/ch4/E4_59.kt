// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.59

package sicp.ch4.solutions

import sicp.ch4.QAnd
import sicp.ch4.QFact
import sicp.ch4.QOr
import sicp.ch4.QPattern
import sicp.ch4.QRule
import sicp.ch4.QVar
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver

// Exercise 4.59: meetings. The weekly meetings are facts; the
// meeting-time rule answers a person's meetings as the whole-company
// meeting or their division's meeting. The Friday query picks the Friday
// meetings, and the Wednesday query for Alyssa answers the whole-company
// meeting first and the computer meeting second.

/** The meeting facts of the exercise. */
internal fun addMeetings(db: QueryDatabase) {
    db.assertFact(QFact(list(sym("meeting"), sym("accounting"), list(sym("Monday"), sym("9am")))))
    db.assertFact(QFact(list(sym("meeting"), sym("administration"), list(sym("Monday"), sym("10am")))))
    db.assertFact(QFact(list(sym("meeting"), sym("computer"), list(sym("Wednesday"), sym("3pm")))))
    db.assertFact(QFact(list(sym("meeting"), sym("administration"), list(sym("Friday"), sym("1pm")))))
    db.assertFact(QFact(list(sym("meeting"), sym("whole-company"), list(sym("Wednesday"), sym("4pm")))))
    db.addRule(
        QRule(
            list(sym("meeting-time"), v("person"), v("day-and-time")),
            QOr(
                listOf(
                    QPattern(list(sym("meeting"), sym("whole-company"), v("day-and-time"))),
                    QAnd(
                        listOf(
                            QPattern(list(sym("job"), v("person"), improper(listOf(v("division")), v("rest")))),
                            QPattern(list(sym("meeting"), v("division"), v("day-and-time"))),
                        ),
                    ),
                ),
            ),
        ),
    )
}

/** The meeting queries with their answers. */
public fun meetingQueries(): List<String> {
    val db = microshaftSystem()
    addMeetings(db)
    val driver = QueryDriver.streaming(db)
    val friday =
        answerLines(
            driver,
            QPattern(list(sym("meeting"), v("division"), list(sym("Friday"), v("time")))),
            listOf(v("division"), v("time")),
        )
    val wednesday =
        answerLines(
            driver,
            QPattern(list(sym("meeting-time"), list(sym("Hacker"), sym("Alyssa"), sym("P")), list(sym("Wednesday"), v("time")))),
            listOf(v("time")),
        )
    return friday + wednesday
}
