// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.71: the delay debate, settled by demand.

package sicp.ch4.solutions

import sicp.ch4.QPattern
import sicp.ch4.QQuery
import sicp.ch4.QVar
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver
import sicp.ch4.renderAnswer

// Exercise 4.71: Louis's undelayed construction against demand-driven
// streams. Building every answer before delivering any diverges on an
// unbounded space: the unanchored query and the married cycle never
// finish construction. Prefixes take answers off the front while the
// rest of the space stays unexplored; the unbounded cycle's prefix runs
// under the loop detector, which bounds its re-entry depth so the take
// terminates on answers the bound admits. Louis's engine has no such
// prefix to pin; the suite pins what the delayed engine delivers.

// Exercise 4.71: demand delivers the first answers; construction never would.

/** Answer lines off the front of the stream: the take bounds exploration. */
internal fun takeAnswerLines(
    driver: QueryDriver,
    query: QQuery,
    variables: List<QVar>,
    count: Int,
): List<String> =
    driver
        .run(query, variables)
        .take(count)
        .flatMap { renderAnswer(it, variables) }
        .toList()

/** Demand-driven prefixes of two unbounded queries. */
public fun delayDebate(): List<String> {
    val stock = microshaftSystem()
    val stream = QueryDriver.streaming(stock)
    val staff = listOf(v("staff-person"), v("boss"))
    val outranked = takeAnswerLines(stream, QPattern(list(sym("outranked-by"), v("staff-person"), v("boss"))), staff, 3)
    val marriedDb = QueryDatabase()
    addMarriedLoop(marriedDb)
    val married = QueryDriver.loopDetecting(marriedDb, 8)
    val cycle = takeAnswerLines(married, QPattern(list(sym("married"), sym("Mickey"), v("who"))), listOf(v("who")), 1)
    return listOf("delayed engine, first three of the unanchored outranked query:") + outranked +
        listOf("married cycle, first answer:") + cycle
}
