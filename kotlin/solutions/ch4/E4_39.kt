// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.39

package sicp.ch4.solutions

// Exercise 4.39: does the order of restrictions affect the answer? No:
// the puzzle keeps its single assignment however the requirements are
// ordered. Does it affect the running time? Not here either -- a candidate
// fails on its first false requirement however the requirements are
// ordered, so the search fails the same 1835 candidates before the
// answer either way. (The book's 1470 belongs to its evaluator; this
// guest search counts failed requirements per attempted candidate.)
// The probes print the backtracks and the answer at the delivered solution.

/** The book's multiple dwelling: distinctness first, restrictions after. */
internal val DWELLING_BOOK_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" +
        """
fun multipleDwelling(): Unit {
    val baker = anIntegerBetween(1L, 5L)
    val cooper = anIntegerBetween(1L, 5L)
    val fletcher = anIntegerBetween(1L, 5L)
    val miller = anIntegerBetween(1L, 5L)
    val smith = anIntegerBetween(1L, 5L)
    requireThat(isDistinct(listOf(baker, cooper, fletcher, miller, smith)))
    requireThat(baker != 5L)
    requireThat(cooper != 1L)
    requireThat(fletcher != 5L)
    requireThat(fletcher != 1L)
    requireThat(miller > cooper)
    requireThat(absLong(smith - fletcher) != 1L)
    requireThat(absLong(fletcher - cooper) != 1L)
    println(showLong(backtracks))
    println(dwellingLine(baker, cooper, fletcher, miller, smith))
}

fun main() {
    budgetCap = 1000000L
    multipleDwelling()
}
        """.trimIndent()

/** The reordered restrictions: adjacent-floor and range checks first,
 * distinctness last. */
internal val DWELLING_REORDERED_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" +
        """
fun multipleDwellingReordered(): Unit {
    val baker = anIntegerBetween(1L, 5L)
    val cooper = anIntegerBetween(1L, 5L)
    val fletcher = anIntegerBetween(1L, 5L)
    val miller = anIntegerBetween(1L, 5L)
    val smith = anIntegerBetween(1L, 5L)
    requireThat(fletcher != 5L)
    requireThat(fletcher != 1L)
    requireThat(absLong(fletcher - cooper) != 1L)
    requireThat(baker != 5L)
    requireThat(cooper != 1L)
    requireThat(miller > cooper)
    requireThat(isDistinct(listOf(baker, cooper, fletcher, miller, smith)))
    requireThat(absLong(smith - fletcher) != 1L)
    println(showLong(backtracks))
    println(dwellingLine(baker, cooper, fletcher, miller, smith))
}

fun main() {
    budgetCap = 1000000L
    multipleDwellingReordered()
}
        """.trimIndent()

/** The puzzle's single answer. => "((baker 3) (cooper 2) (fletcher 4)
 * (miller 5) (smith 1))" */
public fun dwellingAnswer(): String = searchLines(DWELLING_BOOK_PROGRAM)[1]

/** The book order's backtracks to the answer: 1835. */
public fun bookOrderBacktracks(): Long = searchLines(DWELLING_BOOK_PROGRAM)[0].toLong()

/** The reordered restrictions backtrack the same 1835 times: a candidate
 * fails once however the requirements are ordered. */
public fun reorderedBacktracks(): Long = searchLines(DWELLING_REORDERED_PROGRAM)[0].toLong()
