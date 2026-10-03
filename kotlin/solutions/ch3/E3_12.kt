// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.12

package sicp.ch3.exercises

import sicp.runtime.Datum
import sicp.runtime.Empty
import sicp.runtime.PairCell
import sicp.runtime.pair

/**
 * The book's `append` of 2.2.1: a fresh list built by consing each
 * element of `x` onto `y`; no pair of `x` is touched.
 */
public fun append(
    x: Datum,
    y: Datum,
): Datum =
    if (x !is PairCell) {
        y
    } else {
        pair(x.first, append(x.second, y))
    }

/** The last pair of a nonempty proper list. */
public fun lastPair(x: PairCell): PairCell = if (x.second === Empty) x else lastPair(x.second as PairCell)

/** Splice [y] onto the end of [x], returning [x] itself. */
public fun appendBang(
    x: PairCell,
    y: Datum,
): PairCell {
    lastPair(x).second = y
    return x
}
