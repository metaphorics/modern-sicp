// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.43

package sicp.ch2.exercises

/**
 * Louis's version: the map over the rows is the OUTER loop, so every
 * candidate row re-derives the whole (k - 1)-column board set from
 * scratch. The answer set is the same; the work is not.
 */
public fun queensSlow(boardSize: Int): List<List<Int>> {
    fun queenCols(k: Int): List<List<Int>> =
        if (k == 0) {
            listOf(emptyBoard())
        } else {
            flatMapSeq(
                { newRow -> queenCols(k - 1).map { rest -> adjoinPosition(newRow, k, rest) } },
                (1..boardSize).toList(),
            ).filter { safeCol(k, it) }
        }
    return queenCols(boardSize)
}

/** `queensSlow(6)` finds the same 4 solutions as `queens(6)`. */
public fun ex_2_43(): Int = queensSlow(6).size
