// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.77: filters must wait for their bindings.

package sicp.ch4.solutions

import sicp.ch4.QAnd
import sicp.ch4.QGuard
import sicp.ch4.QNot
import sicp.ch4.QPattern
import sicp.ch4.QueryDriver

// Exercise 4.77: why `not` and `lisp-value` must wait. A filter over
// unbound variables decides nothing: the negation's subquery succeeds
// on the open pattern and drops every frame, and the guard has no
// amount to compare and drops every frame. With the bindings in place
// first -- supervision before negation, salary before the comparison
// -- both filters keep exactly the right frames. The delayed
// implementation the exercise designs would postpone the unbound
// filters instead of failing them; the probes pin the naive orders
// that make the delay necessary and the bound orders that need none.

// Exercise 4.77: unbound filters drop everything; bound filters keep their frames.

/** Wrong orders that drop, right orders that keep. */
public fun delayedFilterDemos(): List<String> {
    val db = microshaftSystem()
    val driver = QueryDriver.streaming(db)
    val notFirst =
        answerLines(
            driver,
            QAnd(
                listOf(
                    QNot(QPattern(list(sym("job"), v("x"), list(sym("computer"), sym("programmer"))))),
                    QPattern(list(sym("supervisor"), v("x"), v("y"))),
                ),
            ),
            listOf(v("x"), v("y")),
        )
    val lispFirst =
        answerLines(
            driver,
            QAnd(
                listOf(
                    QGuard({ terms -> termLong(terms[0]) > termLong(terms[1]) }, listOf(v("amount"), sym("30000"))),
                    QPattern(list(sym("salary"), v("who"), v("amount"))),
                ),
            ),
            listOf(v("who"), v("amount")),
        )
    val boundOrder =
        answerLines(
            driver,
            QAnd(
                listOf(
                    QPattern(list(sym("supervisor"), v("x"), v("y"))),
                    QNot(QPattern(list(sym("job"), v("x"), list(sym("computer"), sym("programmer"))))),
                ),
            ),
            listOf(v("x"), v("y")),
        )
    val boundGuard =
        answerLines(
            driver,
            QAnd(
                listOf(
                    QPattern(list(sym("salary"), v("who"), v("amount"))),
                    QGuard({ terms -> termLong(terms[0]) > termLong(terms[1]) }, listOf(v("amount"), sym("30000"))),
                ),
            ),
            listOf(v("who"), v("amount")),
        )
    return listOf("not-first: ${notFirst.size} frame(s), the unbound filter drops everything") +
        listOf("lisp-value-first: ${lispFirst.size} frame(s), the unbound guard drops everything") +
        boundOrder + boundGuard
}
