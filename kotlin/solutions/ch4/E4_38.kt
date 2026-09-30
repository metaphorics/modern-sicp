// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.38

package sicp.ch4.solutions

// Exercise 4.38: the multiple dwelling puzzle without the Smith-Fletcher
// clause. Dropping one restriction widens the solution space from one
// assignment to five; the search finds them all in enumeration order, and
// an ordinary host program enumerating the same space agrees answer for
// answer.

/** The modified puzzle as guest source for the search experiment. */
internal val DWELLING_MODIFIED_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" +
        """
fun multipleDwellingModified(): Unit {
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
    requireThat(absLong(fletcher - cooper) != 1L)
    println(dwellingLine(baker, cooper, fletcher, miller, smith))
}

fun main() {
    budgetCap = 1000000L
    multipleDwellingModified()
}
        """.trimIndent()

/** The puzzle's constraints as an ordinary host predicate. */
private fun acceptsModified(people: List<Int>): Boolean {
    val baker = people[0]
    val cooper = people[1]
    val fletcher = people[2]
    val miller = people[3]
    return people.distinct().size == 5 &&
        baker != 5 &&
        cooper != 1 &&
        fletcher != 5 &&
        fletcher != 1 &&
        miller > cooper &&
        kotlin.math.abs(fletcher - cooper) != 1
}

/** The assignment at [code], baker slowest and smith fastest. */
private fun assignmentAt(code: Int): List<Int> =
    listOf(
        (code / 625) % 5 + 1,
        (code / 125) % 5 + 1,
        (code / 25) % 5 + 1,
        (code / 5) % 5 + 1,
        code % 5 + 1,
    )

/** One host-rendered answer line. */
private fun hostDwellingLine(people: List<Int>): String =
    "((baker " + people[0] + ") (cooper " + people[1] + ") (fletcher " + people[2] +
        ") (miller " + people[3] + ") (smith " + people[4] + "))"

/** Five solutions without the Smith-Fletcher clause.
 * => [((baker 1) (cooper 2) (fletcher 4) (miller 3) (smith 5)),
 * ((baker 1) (cooper 2) (fletcher 4) (miller 5) (smith 3)),
 * ((baker 1) (cooper 4) (fletcher 2) (miller 5) (smith 3)),
 * ((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1)),
 * ((baker 3) (cooper 4) (fletcher 2) (miller 5) (smith 1))] */
public fun modifiedDwellingSolutions(): List<String> = searchLines(DWELLING_MODIFIED_PROGRAM)

/** The same puzzle enumerated by an ordinary host program. */
public fun modifiedDwellingBruteForce(): List<String> {
    val solutions = mutableListOf<String>()
    var code = 0
    while (code < 3125) {
        val people = assignmentAt(code)
        if (acceptsModified(people)) {
            solutions.add(hostDwellingLine(people))
        }
        code = code + 1
    }
    return solutions
}
