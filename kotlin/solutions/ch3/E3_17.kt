// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.17

package sicp.ch3.exercises

import sicp.runtime.Datum
import sicp.runtime.PairCell

/**
 * A corrected `countDistinctPairs`: the same traversal as the visit count,
 * but a shared pair is counted on its first visit only. A local list is
 * scanned with `===`, so identity membership stays explicit.
 */
public fun countDistinctPairs(x: Datum): Int {
    val seen = mutableListOf<PairCell>()

    fun walk(v: Datum): Int =
        if (v !is PairCell) {
            0
        } else if (seen.any { it === v }) {
            0
        } else {
            seen.add(v)
            walk(v.first) + walk(v.second) + 1
        }

    return walk(x)
}
