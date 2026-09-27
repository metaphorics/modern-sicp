// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.14

package sicp.ch3.exercises

import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.Value
import sicp.runtime.setCdr

/**
 * The book's `mystery`: reverse the chain `x` in place. The internal
 * `loop` keeps the old `cdr` in `temp` while it repoints the pair's `cdr`
 * at the cells already passed, so `v` ends as the last cell alone and the
 * returned value is the old last cell.
 */
public fun mystery(x: VPair): VPair {
    fun loop(
        x: Value,
        y: Value,
    ): VPair =
        if (x !is VPair) {
            y as VPair
        } else {
            val temp = x.cdr
            x.setCdr(y)
            loop(temp, x)
        }
    return loop(x, VNil)
}
