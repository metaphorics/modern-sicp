// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.42

package sicp.ch4.solutions

// Exercise 4.42: the Liars puzzle. Each girl makes one claim; exactly one
// of every pair of claims is true. The requirement is a parity check over
// each pair, and the search finds the one placement of the five girls'
// grades that survives all five checks.

/** The Liars puzzle as guest source for the search experiment. */
internal val LIARS_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" +
        """
fun oneTrue(p: Boolean, q: Boolean): Boolean = p != q

fun liars(): Unit {
    val betty = anIntegerBetween(1L, 5L)
    val ethel = anIntegerBetween(1L, 5L)
    val joan = anIntegerBetween(1L, 5L)
    val kitty = anIntegerBetween(1L, 5L)
    val mary = anIntegerBetween(1L, 5L)
    requireThat(isDistinct(listOf(betty, ethel, joan, kitty, mary)))
    requireThat(oneTrue(kitty == 2L, betty == 3L))
    requireThat(oneTrue(ethel == 1L, joan == 2L))
    requireThat(oneTrue(joan == 3L, ethel == 5L))
    requireThat(oneTrue(kitty == 2L, mary == 4L))
    requireThat(oneTrue(mary == 4L, betty == 1L))
    println(
        "((betty " + showLong(betty) + ") (ethel " + showLong(ethel) + ") (joan " + showLong(joan) +
            ") (kitty " + showLong(kitty) + ") (mary " + showLong(mary) + "))",
    )
}

fun main() {
    budgetCap = 1000000L
    liars()
}
        """.trimIndent()

/** One placement survives the liars' statements.
 * => [((betty 3) (ethel 5) (joan 2) (kitty 1) (mary 4))] */
public fun liarsSolutions(): List<String> = searchLines(LIARS_PROGRAM)
