// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.78: the query as a nondeterministic program.

package sicp.ch4.solutions

import sicp.ch4.QPattern
import sicp.ch4.QueryDriver
import sicp.ch4.SearchModule

// Exercise 4.78: the query language as nondeterministic search. The
// same question runs two ways: as a query over the data base, answered
// in data-base order, and as a search program choosing staff indices
// and demanding the supervisor relation from a guest table. Each
// entered alternative is one choice, counted by a permanent write so
// the cumulative count survives backtracking; the answers arrive with
// the count that delivered them. Both engines agree on the answers;
// the engine's own end-of-run counter agrees with the guest count.

// Exercise 4.78: one question, two engines, agreeing answers and counts.

/** Ben's direct reports, both engines' way. */
internal val AMB_SEARCH_PROGRAM: String =
    """
var seen: Long = 0L

fun showLong(n: Long): String = "${'$'}{n}"

fun staffName(i: Long): String {
    if (i == 0L) {
        return "Hacker Alyssa P"
    }
    if (i == 1L) {
        return "Fect Cy D"
    }
    if (i == 2L) {
        return "Tweakit Lem E"
    }
    if (i == 3L) {
        return "Reasoner Louis"
    }
    if (i == 4L) {
        return "Bitdiddle Ben"
    }
    if (i == 5L) {
        return "Scrooge Eben"
    }
    if (i == 6L) {
        return "Cratchet Robert"
    }
    return "Aull DeWitt"
}

fun isDirectReport(i: Long): Boolean {
    if (i == 0L) {
        return true
    }
    if (i == 1L) {
        return true
    }
    if (i == 2L) {
        return true
    }
    return false
}

fun noteChoice(): Unit {
    setPermanent { seen = seen + 1L }
}

fun main() {
    val i = choose(0L, 1L, 2L, 3L, 4L, 5L, 6L, 7L)
    noteChoice()
    demand(isDirectReport(i))
    println(staffName(i) + " (choice " + showLong(seen) + ")")
}
    """.trimIndent()

/** The nondeterministic answers with their choice counts, the stream
 * answers, and the agreed total. */
public fun ambQueryDemos(): List<String> {
    val amb = searchLines(AMB_SEARCH_PROGRAM)
    val db = microshaftSystem()
    val stream =
        answerLines(
            QueryDriver.streaming(db),
            QPattern(list(sym("supervisor"), v("x"), list(sym("Bitdiddle"), sym("Ben")))),
            listOf(v("x")),
        )
    val total = searchRun(AMB_SEARCH_PROGRAM).choices
    return amb + stream + listOf("total choices: $total")
}
