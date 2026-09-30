// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.44

package sicp.ch4.solutions

// Exercise 4.44: the eight-queens puzzle. `queen-cols` builds the board
// column by column, newest column first, and every placement meets the
// safety check against the columns already placed: same row and both
// diagonals. The search answers the first solution of each board size in
// the generator's own order, and the probe's backtrack counter records
// the work the safety check saves.

/** The queens generator: board rows, newest column first. */
internal val QUEENS_SOURCE: String =
    AMB_BASE_PRELUDE + "\n" +
        """
fun rowLine(rows: List<Long>): String {
    var out = "("
    var index = 0
    while (index < rows.size) {
        if (index > 0) {
            out = out + " "
        }
        out = out + showLong(rows.get(index))
        index = index + 1
    }
    return out + ")"
}

fun isSafe(positions: List<Long>): Boolean {
    val newest = positions.get(0)
    var rest = positions.drop(1)
    var distance = 1L
    while (rest.size > 0) {
        val row = rest.get(0)
        if (row == newest) {
            return false
        }
        if (absLong(row - newest) == distance) {
            return false
        }
        rest = rest.drop(1)
        distance = distance + 1L
    }
    return true
}

fun queenCols(k: Long, boardSize: Long): List<Long> {
    if (k == 0L) {
        return emptyList()
    }
    val previous = queenCols(k - 1L, boardSize)
    val row = anIntegerBetween(1L, boardSize)
    val candidate = listOf(row) + previous
    requireThat(isSafe(candidate))
    return candidate
}
        """.trimIndent()

/** One queens run of [boardSize] printing the first solution's rows. */
private fun queensFirstProgram(boardSize: Int): String =
    QUEENS_SOURCE + "\n" +
        """
fun main() {
    budgetCap = 1000000L
    val solution = queenCols(${boardSize}L, ${boardSize}L)
    println(rowLine(solution))
}
        """.trimIndent()

/** One queens run of [boardSize] printing its backtracks at the solution. */
private fun queensCountedProgram(boardSize: Int): String =
    QUEENS_SOURCE + "\n" +
        """
fun main() {
    budgetCap = 1000000L
    val solution = queenCols(${boardSize}L, ${boardSize}L)
    println(showLong(backtracks))
}
        """.trimIndent()

/** The first solution for [boardSize], newest column first.
 * queensFirst(8) => "(4 2 7 3 6 8 5 1)";
 * queensFirst(4) => "(3 1 4 2)"; queensFirst(6) => "(5 3 1 6 4 2)" */
public fun queensFirst(boardSize: Int): String = searchLines(queensFirstProgram(boardSize)).first()

/** Backtracks to the first solution of [boardSize].
 * queensBacktracks(4) => 22; (5) => 10; (6) => 165; (8) => 868 */
public fun queensBacktracks(boardSize: Int): Long = searchLines(queensCountedProgram(boardSize)).first().toLong()
