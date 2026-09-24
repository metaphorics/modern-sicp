// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.15

package sicp.ch3.exercises

import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.setCar

/**
 * The book's `set-to-wow!`: replace the car pointer of `x`'s first pair
 * with the symbol `wow` and return `x`. On the shared `z1` the change
 * shows through both the car and the cdr; on the unshared `z2` only the
 * car sees it.
 */
public fun setToWow(x: VPair): VPair {
    (x.car as VPair).setCar(VSym("wow"))
    return x
}
