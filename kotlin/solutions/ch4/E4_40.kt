// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.40

package sicp.ch4.solutions

// Exercise 4.40: generate and test. Before the distinctness requirement
// there are 5 * 5 * 5 * 5 * 5 = 3125 assignments of people to floors;
// after it there are 5! = 120. The probes count both spaces by
// enumerating them (the counter is a permanent write and `ifFail` prints
// it once the enumeration exhausts). The exercise's second question is
// the one that matters for the search: most of the 120 are rejected by
// the other restrictions only after every floor is chosen. Interleaving
// each restriction with its own choice prunes the space as it is built,
// and the backtracks to the first answer drop from 1470 to 210 while the
// answer stays the same.

/** The assignment-counting space: all tuples, then the distinct ones. */
internal val DWELLING_SPACE_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" +
        """
var assignments: Long = 0L

fun countAssignment(baker: Long, cooper: Long, fletcher: Long, miller: Long, smith: Long): Unit {
    setPermanent { assignments = assignments + 1L }
}

fun dwellingsAll(): Unit {
    val baker = anIntegerBetween(1L, 5L)
    val cooper = anIntegerBetween(1L, 5L)
    val fletcher = anIntegerBetween(1L, 5L)
    val miller = anIntegerBetween(1L, 5L)
    val smith = anIntegerBetween(1L, 5L)
    countAssignment(baker, cooper, fletcher, miller, smith)
}

fun dwellingsDistinct(): Unit {
    val baker = anIntegerBetween(1L, 5L)
    val cooper = anIntegerBetween(1L, 5L)
    val fletcher = anIntegerBetween(1L, 5L)
    val miller = anIntegerBetween(1L, 5L)
    val smith = anIntegerBetween(1L, 5L)
    requireThat(isDistinct(listOf(baker, cooper, fletcher, miller, smith)))
    countAssignment(baker, cooper, fletcher, miller, smith)
}
        """.trimIndent()

/** The pruned generator: each restriction meets its own choice. */
internal val DWELLING_PRUNED_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" +
        """
fun multipleDwellingFaster(): Unit {
    val baker = anIntegerBetween(1L, 5L)
    requireThat(baker != 5L)
    val cooper = anIntegerBetween(1L, 5L)
    requireThat(cooper != 1L)
    val fletcher = anIntegerBetween(1L, 5L)
    requireThat(fletcher != 5L)
    requireThat(fletcher != 1L)
    requireThat(absLong(fletcher - cooper) != 1L)
    val miller = anIntegerBetween(1L, 5L)
    requireThat(miller > cooper)
    val smith = anIntegerBetween(1L, 5L)
    requireThat(isDistinct(listOf(baker, cooper, fletcher, miller, smith)))
    requireThat(absLong(smith - fletcher) != 1L)
    println(showLong(backtracks))
    println(dwellingLine(baker, cooper, fletcher, miller, smith))
}

fun main() {
    budgetCap = 1000000L
    multipleDwellingFaster()
}
        """.trimIndent()

/** 3125 assignments before the distinctness requirement. */
public fun assignmentsBeforeDistinct(): Int =
    searchLines(
        DWELLING_SPACE_PROGRAM + "\n" +
            """
fun main() {
    budgetCap = 1000000L
    ifFail(
        { dwellingsAll() },
        { println(showLong(assignments)) },
    )
}
            """.trimIndent(),
    ).last().toInt()

/** 120 assignments after it. */
public fun assignmentsAfterDistinct(): Int =
    searchLines(
        DWELLING_SPACE_PROGRAM + "\n" +
            """
fun main() {
    budgetCap = 1000000L
    ifFail(
        { dwellingsDistinct() },
        { println(showLong(assignments)) },
    )
}
            """.trimIndent(),
    ).last().toInt()

/** The unpruned generator's backtracks to the first answer: 1470. */
public fun naiveBacktracksToFirst(): Long = searchLines(DWELLING_BOOK_PROGRAM)[0].toLong()

/** The pruned generator's backtracks to the first answer: 210. */
public fun prunedBacktracksToFirst(): Long = searchLines(DWELLING_PRUNED_PROGRAM)[0].toLong()

/** The pruned program answers the same assignment.
 * => "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))" */
public fun prunedAnswer(): String = searchLines(DWELLING_PRUNED_PROGRAM)[1]
