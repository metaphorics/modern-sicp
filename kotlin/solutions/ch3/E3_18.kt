// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.18

package sicp.ch3.exercises

import sicp.runtime.Datum
import sicp.runtime.PairCell

/**
 * Cycle detection by remembering: follow the second-field chain and keep
 * visited `PairCell` objects in a list scanned with `===`. Seeing a cell
 * twice means the chain loops; reaching `Empty` means it terminates.
 */
public fun containsCycle(x: Datum): Boolean {
    val seen = mutableListOf<PairCell>()
    var cursor = x
    while (cursor is PairCell) {
        if (seen.any { it === cursor }) {
            return true
        }
        seen.add(cursor)
        cursor = cursor.second
    }
    return false
}
