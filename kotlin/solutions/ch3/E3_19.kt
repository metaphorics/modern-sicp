// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.19

package sicp.ch3.exercises

import sicp.runtime.VPair
import sicp.runtime.Value

/**
 * Cycle detection in constant space (Floyd's tortoise and hare): the hare
 * takes two cdr steps for the tortoise's one. If a cycle exists, the two
 * cursors meet inside it; if the hare falls off the end onto `VNil`, the
 * chain is acyclic. No memory of visited pairs at all.
 */
public fun containsCycleConstantSpace(x: Value): Boolean {
    var tortoise: Value = x
    var hare: Value = x
    while (hare is VPair) {
        hare = hare.cdr
        if (hare !is VPair) {
            return false
        }
        hare = hare.cdr
        tortoise = (tortoise as VPair).cdr
        if (hare === tortoise) {
            return true
        }
    }
    return false
}
