// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.19

package sicp.ch3.exercises

import sicp.runtime.Datum
import sicp.runtime.PairCell

/**
 * Floyd's cycle detector advances `hare` twice and `tortoise` once along
 * the second-field chain. They meet inside a loop; if `hare` reaches a
 * non-pair datum, the chain is finite. The algorithm keeps constant space.
 */
public fun containsCycleConstantSpace(x: Datum): Boolean {
    var tortoise: Datum = x
    var hare: Datum = x
    while (hare is PairCell) {
        hare = hare.second
        if (hare !is PairCell) {
            return false
        }
        hare = hare.second
        tortoise = (tortoise as PairCell).second
        if (hare === tortoise) {
            return true
        }
    }
    return false
}
