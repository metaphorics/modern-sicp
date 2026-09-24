// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.23

package sicp.ch2.exercises

import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.Value
import sicp.runtime.vlist

/**
 * The book's `for-each`: applies [action] to every element of [items],
 * from left to right, for its side effects; it returns nothing useful,
 * like a Kotlin `Unit`-returning procedure.
 */
public fun forEachValue(
    items: Value,
    action: (Value) -> Unit,
) {
    if (items !is VNil) {
        action(carOf(items))
        forEachValue(cdrOf(items), action)
    }
}

/** The values a recording action collects while `forEachValue` runs over `(57 321 88)`. */
public fun ex_2_23(): List<Long> {
    val seen = mutableListOf<Long>()
    forEachValue(vlist(VInt(57L), VInt(321L), VInt(88L))) { v -> seen.add(numOf(v)) }
    return seen
}
