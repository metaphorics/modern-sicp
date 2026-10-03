// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.14

package sicp.ch3.exercises

import sicp.runtime.Datum
import sicp.runtime.Empty
import sicp.runtime.PairCell

/**
 * Reverse the pair chain in place. Each step retains the old second field,
 * points the current cell at the reversed prefix, then advances.
 */
public fun mystery(x: PairCell): PairCell {
    fun loop(
        x: Datum,
        y: Datum,
    ): PairCell =
        if (x !is PairCell) {
            y as PairCell
        } else {
            val next = x.second
            x.second = y
            loop(next, x)
        }
    return loop(x, Empty)
}
