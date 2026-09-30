// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.44

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.44: the queens puzzle as a nondeterministic program. Each
 * column k extends the answer for k-1 with one `anIntegerBetween`
 * row choice and requires the new queen to be safe; the positions are
 * kept newest-column-first, so the printed answer lists row of column
 * n down to row of column 1. The backtrack count is the edition's
 * metric: one per failure resumption that delivers an alternative.
 *
 * Expected answers: the first solutions for board sizes 8, 4, and 6 are
 * [4, 2, 7, 3, 6, 8, 5, 1], [3, 1, 4, 2], and [5, 3, 1, 6, 4, 2], costing 868, 22, and
 * 165 backtracks to find.
 */
public fun queensFirst(boardSize: Int): String = throw PendingSolution()

public fun queensBacktracks(boardSize: Int): Long = throw PendingSolution()
