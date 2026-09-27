// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.18

package sicp.ch3.exercises

import sicp.runtime.VPair
import sicp.runtime.Value

/**
 * Cycle detection by remembering: walk the cdr chain, keeping the pairs
 * already visited in a plain list scanned with `===` (there is no
 * identity-keyed set in the standard library). Meeting a pair a second
 * time means some cdr chain loops; reaching `VNil` means it does not.
 */
public fun containsCycle(x: Value): Boolean {
    val seen = mutableListOf<VPair>()
    var cursor = x
    while (cursor is VPair) {
        if (seen.any { it === cursor }) {
            return true
        }
        seen.add(cursor)
        cursor = cursor.cdr
    }
    return false
}
