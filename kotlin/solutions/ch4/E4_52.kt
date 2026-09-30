// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.52

package sicp.ch4.solutions

// Exercise 4.52: `if-fail`. The experiment's `ifFail` enters its second
// body exactly when the first exhausts its answers: with all-odd choices
// the filter finds nothing and the alternative answers, and with an even
// choice in the list the filter answers first and the alternative follows
// the exhaustion.

/** The all-odd session: the filter finds nothing. */
internal val IF_FAIL_ALL_ODD_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" +
        """
fun main() {
    budgetCap = 1000000L
    ifFail(
        {
            val x = anElementOf(listOf(1L, 3L, 5L))
            requireThat(isEven(x))
            println(showLong(x))
        },
        { println("all-odd") },
    )
}
        """.trimIndent()

/** The eight session: one even choice answers, then the alternative. */
internal val IF_FAIL_EIGHT_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" +
        """
fun main() {
    budgetCap = 1000000L
    ifFail(
        {
            val x = anElementOf(listOf(1L, 3L, 5L, 8L))
            requireThat(isEven(x))
            println(showLong(x))
        },
        { println("all-odd") },
    )
}
        """.trimIndent()

/** All odd means the alternative answers. => "all-odd\n" */
public fun ifFailAllOddTranscript(): String = searchLines(IF_FAIL_ALL_ODD_PROGRAM).joinToString(separator = "\n", postfix = "\n")

/** One even choice answers first, then the alternative answers after the
 * exhaustion. => "8\nall-odd\n" */
public fun ifFailEightTranscript(): String = searchLines(IF_FAIL_EIGHT_PROGRAM).joinToString(separator = "\n", postfix = "\n")
