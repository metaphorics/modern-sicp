// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.12

package sicp.ch3.exercises

import sicp.runtime.Datum
import sicp.runtime.PairCell
import sicp.runtime.PendingSolution

/**
 * Exercise 3.12: `append` builds a fresh pair spine in front of `y`, leaving
 * every pair in `x` untouched. `appendBang` changes the final cell of `x`
 * to point at `y` and returns the original head. Compare their sharing by
 * following `PairCell.second` references and checking object identity.
 */
public fun append(
    x: Datum,
    y: Datum,
): Datum = throw PendingSolution()

/** The last pair of a nonempty proper list. */
public fun lastPair(x: PairCell): PairCell = throw PendingSolution()

/** Splice [y] onto the end of [x] by mutation. */
public fun appendBang(
    x: PairCell,
    y: Datum,
): PairCell = throw PendingSolution()
