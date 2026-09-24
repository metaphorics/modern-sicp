// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.17

package sicp.ch3.exercises

import sicp.runtime.VPair
import sicp.runtime.Value

/**
 * A corrected `count-pairs`: the same traversal as Ben's, but a shared
 * pair is counted the first time only. Kotlin's standard library has no
 * identity-keyed set, so the auxiliary structure is a plain list scanned
 * with `===` (the book's `eq?`).
 */
public fun countDistinctPairs(x: Value): Int {
    val seen = mutableListOf<VPair>()

    fun walk(v: Value): Int =
        if (v !is VPair) {
            0
        } else if (seen.any { it === v }) {
            0
        } else {
            seen.add(v)
            walk(v.car) + walk(v.cdr) + 1
        }

    return walk(x)
}
