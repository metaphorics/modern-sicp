// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.42

package sicp.ch2.exercises

/** The empty board: no queens placed yet. */
public fun emptyBoard(): List<Int> = emptyList()

/** The book's `adjoin-position`: place a queen in row [newRow] of column [k]; columns index from 1. */
public fun adjoinPosition(
    newRow: Int,
    k: Int,
    positions: List<Int>,
): List<Int> = positions + newRow

/**
 * Whether the queen in column [k] is safe with respect to the queens in
 * the earlier columns: no shared row and no shared diagonal.
 */
public fun safeCol(
    k: Int,
    positions: List<Int>,
): Boolean =
    (1 until k).none { column ->
        val other = positions[column - 1]
        val row = positions[k - 1]
        other == row || kotlin.math.abs(other - row) == k - column
    }

/**
 * The book's `queens`: recursively find every safe way to place k queens
 * in the first k columns by adjoining each row choice to each (k - 1)-
 * column board and keeping the safe ones.
 */
public fun queens(boardSize: Int): List<List<Int>> {
    fun queenCols(k: Int): List<List<Int>> =
        if (k == 0) {
            listOf(emptyBoard())
        } else {
            flatMapSeq(
                { rest -> (1..boardSize).map { newRow -> adjoinPosition(newRow, k, rest) } },
                queenCols(k - 1),
            ).filter { safeCol(k, it) }
        }
    return queenCols(boardSize)
}

/** `queens(8).size`, the number of solutions to the eight-queens puzzle. */
public fun ex_2_42(): Int = queens(8).size
