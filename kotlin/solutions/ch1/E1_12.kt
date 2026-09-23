// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.12

package sicp.ch1.exercises

/** Off the triangle is 0; the two edges are 1; inside, the sum of the two entries above. */
public fun pascal(
    row: Int,
    col: Int,
): Long =
    when {
        col < 0 || col > row -> 0L
        col == 0 || col == row -> 1L
        else -> pascal(row - 1, col - 1) + pascal(row - 1, col)
    }

public fun ex_1_12(row: Int): List<Long> = (0..row).map { pascal(row, it) }
