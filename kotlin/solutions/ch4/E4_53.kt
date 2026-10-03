// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.53

package sicp.ch4.solutions

// Exercise 4.53: `permanent-set!` with `if-fail`. The book's program
// collects every prime-sum pair of two lists and then fails on purpose:
// the always-failing choice exhausts the first body, the collected pairs
// survive it because their accumulation is a permanent write, and the
// second body answers the collection. The pairs appear newest first
// because the collection prepends.

/** The pair-collecting session. */
internal val PRIME_PAIRS_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" +
        """
var pairItems: List<String> = emptyList()

fun renderPairs(items: List<String>): String {
    var out = "("
    var index = 0
    while (index < items.size) {
        if (index > 0) {
            out = out + " "
        }
        out = out + items.get(index)
        index = index + 1
    }
    return out + ")"
}

fun primeSumPair(): Unit {
    val a = anElementOf(listOf(1L, 3L, 5L, 8L))
    val b = anElementOf(listOf(20L, 35L, 110L))
    requireThat(isPrime(a + b))
    setPermanent { pairItems = listOf(showPair(a, b)) + pairItems }
    demand(false)
}

fun main() {
    budgetCap = 1000000L
    ifFail(
        { primeSumPair() },
        { println(renderPairs(pairItems)) },
    )
}
        """.trimIndent()

/** The accumulated pairs survive the final failure.
 * => "((8 35) (3 110) (3 20))" */
public fun pairsResult(): String = searchLines(PRIME_PAIRS_PROGRAM).last()
