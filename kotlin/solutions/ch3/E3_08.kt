// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.8

package sicp.ch3.exercises

/** Returns its argument the first time it is ever called, and 0 on every later call: a witness for call order. */
public fun makeF(): (Int) -> Int {
    var firstCall = true
    return { x ->
        if (firstCall) {
            firstCall = false
            x
        } else {
            0
        }
    }
}
