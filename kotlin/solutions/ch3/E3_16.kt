// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.16

package sicp.ch3.exercises

import sicp.runtime.VPair
import sicp.runtime.Value

/**
 * Ben's `count-pairs` from the exercise statement, kept exactly as the
 * book's incorrect recursion: every visit counts its pair once, so a
 * shared pair is counted once per path that reaches it.
 */
public fun countPairs(x: Value): Int =
    if (x !is VPair) {
        0
    } else {
        countPairs(x.car) + countPairs(x.cdr) + 1
    }
