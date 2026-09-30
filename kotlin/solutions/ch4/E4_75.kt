// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.75: the unique quantifier over matches.

package sicp.ch4.solutions

import sicp.ch4.QAnd
import sicp.ch4.QPattern
import sicp.ch4.QUnique
import sicp.ch4.QueryDriver

// Exercise 4.75: `unique` keeps exactly the frames its subquery extends
// in one way. Wizardry has one holder, so the frame survives;
// programmer has two, so nothing survives. Joined with job facts, the
// quantifier keeps the singly-held jobs; joined with supervision, it
// keeps the bosses with exactly one report.

// Exercise 4.75: one extension survives; two or zero drop the frame.

/** The singleton tests, the singly-held jobs, the singly-led bosses. */
public fun uniqueDemos(): List<String> {
    val db = microshaftSystem()
    val driver = QueryDriver.streaming(db)
    val wizard =
        answerLines(
            driver,
            QUnique(QPattern(list(sym("job"), v("x"), list(sym("computer"), sym("wizard"))))),
            listOf(v("x")),
        )
    val programmer =
        answerLines(
            driver,
            QUnique(QPattern(list(sym("job"), v("x"), list(sym("computer"), sym("programmer"))))),
            listOf(v("x")),
        )
    val filled =
        answerLines(
            driver,
            QAnd(
                listOf(
                    QPattern(list(sym("job"), v("x"), v("j"))),
                    QUnique(QPattern(list(sym("job"), v("anyone"), v("j")))),
                ),
            ),
            listOf(v("x"), v("j")),
        )
    val singleton =
        answerLines(
            driver,
            QAnd(
                listOf(
                    QPattern(list(sym("supervisor"), v("person"), v("boss"))),
                    QUnique(QPattern(list(sym("supervisor"), v("underling"), v("boss")))),
                ),
            ),
            listOf(v("person"), v("boss")),
        )
    return listOf("unique (job ?x (computer wizard)):") + wizard +
        listOf("unique (job ?x (computer programmer)): ${programmer.size} answer(s)") +
        listOf("jobs held by exactly one:") + filled +
        listOf("bosses with exactly one report:") + singleton
}
