// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.51

package sicp.ch4.solutions

// Exercise 4.51: `permanent-set!`. The book's session asks what survives
// backtracking: with an ordinary assignment each failed trial rolls its
// write back and every answer shows the same count; with a permanent
// write the count accumulates across failed trials and survives. The
// probes take the book's three answers and then observe the count, which
// the run reports after the search exhausts. (The driver-loop transcript
// of the removed evaluator -- its prompts and `try-again` lines -- is
// gone with it: engines emit no prompts, so the observable is the answer
// stream itself.)

/** The counting session shared by both assignments. */
internal val COUNTING_PRELUDE: String =
    AMB_BASE_PRELUDE + "\n" +
        """
var count: Long = 0L

var delivered: Long = 0L

fun recordAnswer(x: String, y: String): Unit {
    requireThat(x != y)
    requireThat(delivered < 3L)
    setPermanent { delivered = delivered + 1L }
    println("(" + x + " " + y + " " + showLong(count) + ")")
}

fun pairedCountsPermanent(): Unit {
    val x = anElementOfString(listOf("a", "b", "c"))
    val y = anElementOfString(listOf("a", "b", "c"))
    if (delivered < 3L) {
        setPermanent { count = count + 1L }
    }
    recordAnswer(x, y)
}

fun pairedCountsOrdinary(): Unit {
    val x = anElementOfString(listOf("a", "b", "c"))
    val y = anElementOfString(listOf("a", "b", "c"))
    count = count + 1L
    recordAnswer(x, y)
}
        """.trimIndent()

/** The permanent-write session and its final count. */
private fun permanentProbe(): String =
    COUNTING_PRELUDE + "\n" +
        """
fun main() {
    budgetCap = 1000000L
    ifFail(
        { pairedCountsPermanent() },
        { println(showLong(count)) },
    )
}
        """.trimIndent()

/** The ordinary-assignment session and its final count. */
private fun setBangProbe(): String =
    COUNTING_PRELUDE + "\n" +
        """
fun main() {
    budgetCap = 1000000L
    ifFail(
        { pairedCountsOrdinary() },
        { println(showLong(count)) },
    )
}
        """.trimIndent()

/** The book's three answers and the count they leave behind.
 * => "(a b 2)\n(a c 3)\n(b a 4)\n4\n" */
public fun permanentSetTranscript(): String = searchLines(permanentProbe()).joinToString(separator = "\n", postfix = "\n")

/** The ordinary assignment rolls every failed trial back.
 * => "(a b 1)\n(a c 1)\n(b a 1)\n0\n" */
public fun setBangTranscript(): String = searchLines(setBangProbe()).joinToString(separator = "\n", postfix = "\n")
