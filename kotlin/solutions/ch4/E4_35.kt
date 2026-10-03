// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.35

package sicp.ch4.solutions

// Exercise 4.35: an-integer-between and the Pythagorean triples. The
// search experiment's `choose`/`demand` realize the section's generator
// exactly: `anIntegerBetween` commits left to right, a failed `requireThat`
// backtracks to the most recent untried alternative, and the run explores
// to exhaustion, so every triple between 1 and 20 arrives in search order.

/** Exercise 4.35's triples procedure, as guest source. */
internal val TRIPLES_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" +
        """
fun tripleLine(i: Long, j: Long, k: Long): String = "(" + showLong(i) + " " + showLong(j) + " " + showLong(k) + ")"

fun aPythagoreanTripleBetween(low: Long, high: Long): Unit {
    val i = anIntegerBetween(low, high)
    val j = anIntegerBetween(i, high)
    val k = anIntegerBetween(j, high)
    requireThat(i * i + j * j == k * k)
    println(tripleLine(i, j, k))
}

fun main() {
    budgetCap = 1000000L
    aPythagoreanTripleBetween(1L, 20L)
}
        """.trimIndent()

/** The triples between 1 and 20, in the search's own order, then
 * exhaustion.
 * => [(3 4 5), (5 12 13), (6 8 10), (8 15 17), (9 12 15), (12 16 20)] */
public fun triplesBetween20(): List<String> = searchLines(TRIPLES_PROGRAM)
