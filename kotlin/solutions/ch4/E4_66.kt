// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.66

package sicp.ch4.solutions

import sicp.ch4.QAnd
import sicp.ch4.QPattern
import sicp.ch4.QVar
import sicp.ch4.QueryDriver
import sicp.ch4.reify

// Exercise 4.66: accumulation over frames. The wheel query answers
// Warbucks once per supervising chain, so summing salaries over its
// frames counts him four times: 660000 where the distinct people cost
// 210000. The book's programmer query has one frame per worker and sums
// correctly to 75000. The lesson is the frames, not the answers: an
// accumulator over a stream must know whether the stream's multiplicity
// is meaningful.

/** The salary index of the Microshaft people. */
internal val microshaftSalaries: Map<String, Long> =
    mapOf(
        "Bitdiddle Ben" to 60000L,
        "Hacker Alyssa P" to 40000L,
        "Fect Cy D" to 35000L,
        "Tweakit Lem E" to 25000L,
        "Reasoner Louis" to 30000L,
        "Warbucks Oliver" to 150000L,
        "Scrooge Eben" to 75000L,
        "Cratchet Robert" to 18000L,
        "Aull DeWitt" to 25000L,
    )

/** The sums of the exercise, in the book's order. */
public fun salarySums(): List<String> {
    val db = microshaftSystem()
    val driver = QueryDriver.streaming(db)
    val computerStaff =
        driver
            .run(
                QAnd(
                    listOf(
                        QPattern(list(sym("job"), v("person"), list(sym("computer"), sym("programmer")))),
                        QPattern(list(sym("salary"), v("person"), v("amount"))),
                    ),
                ),
                listOf(v("person"), v("amount")),
            ).toList()
    val bookSum = computerStaff.sumOf { frame -> termLong(reify(v("amount"), frame)) }
    val wheelFrames = driver.run(QPattern(list(sym("wheel"), v("who"))), listOf(v("who"))).toList()
    val wheelPeople = wheelFrames.map { frame -> personKey(reify(v("who"), frame)) }
    val wheelSum = wheelPeople.sumOf { microshaftSalaries.getValue(it) }
    val distinctSum = wheelPeople.toSet().sumOf { microshaftSalaries.getValue(it) }
    return listOf(
        "sum over the book's query = $bookSum",
        "Ben's scheme on the wheel query = $wheelSum (duplicate frames count)",
        "salvage, distinct answers only = $distinctSum",
    )
}
