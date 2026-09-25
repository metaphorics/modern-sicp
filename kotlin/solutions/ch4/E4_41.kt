// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.41

package sicp.ch4.solutions

// Exercise 4.41: the multiple dwelling puzzle as an ordinary Kotlin
// program. The solver walks the assignment grid in the nondeterministic
// program's generation order -- baker major, each floor ascending, so a
// grid code is a mixed-radix number with baker as the most significant
// digit -- and answers the first candidate that survives the puzzle's
// restrictions. Every one of the 5^5 = 3125 candidates is tested, because
// the restrictions can only run after all five floors are assigned; the
// amb evaluator's restricted variants (4.39, 4.40) prune during
// generation instead, and this plain solver is the measure of what that
// pruning saves. The survivor sits at grid code 1470, so the plain walk
// tests 1471 candidates before it answers.

private const val TOP_FLOOR = 5
private const val BOTTOM_FLOOR = 1
private const val FLOORS = 5
private const val GRID = 3125

/** The assignment a grid code stands for, baker major. */
private fun assignmentOf(code: Int): List<Int> {
    var rest = code
    val floors = MutableList(FLOORS) { BOTTOM_FLOOR }
    for (person in FLOORS - 1 downTo 0) {
        floors[person] = BOTTOM_FLOOR + rest % FLOORS
        rest /= FLOORS
    }
    return floors
}

/** The puzzle's restrictions over one complete assignment. */
private fun satisfies(floors: List<Int>): Boolean {
    val (baker, cooper, fletcher, miller, smith) = floors
    if (floors.toSet().size != FLOORS) return false
    if (baker == TOP_FLOOR) return false
    if (cooper == BOTTOM_FLOOR) return false
    if (fletcher == TOP_FLOOR || fletcher == BOTTOM_FLOOR) return false
    if (miller <= cooper) return false
    if (kotlin.math.abs(smith - fletcher) == 1) return false
    if (kotlin.math.abs(fletcher - cooper) == 1) return false
    return true
}

/** The first surviving assignment and the candidates tested before it,
 * or null when nothing survives. */
private fun firstSurviving(): Pair<Int, List<Int>>? {
    for (code in 0 until GRID) {
        val candidate = assignmentOf(code)
        if (satisfies(candidate)) {
            return Pair(code + 1, candidate)
        }
    }
    return null
}

/** The assignment the plain solver answers first, in the book's listing
 * shape. => "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))" */
public fun kotlinSolverAnswer(): String {
    val surviving = firstSurviving() ?: throw IllegalStateException("the puzzle has no solution")
    val (baker, cooper, fletcher, miller, smith) = surviving.second
    return "((baker $baker) (cooper $cooper) (fletcher $fletcher) " +
        "(miller $miller) (smith $smith))"
}

/** The candidates the plain solver tests before the answer survives:
 * 1471 of the 3125-candidate grid. => 1471 */
public fun kotlinSolverTests(): Int {
    val surviving = firstSurviving() ?: throw IllegalStateException("the puzzle has no solution")
    return surviving.first
}
