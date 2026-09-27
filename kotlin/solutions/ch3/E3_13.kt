// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.13

package sicp.ch3.exercises

import sicp.runtime.VPair
import sicp.runtime.setCdr

/**
 * The book's `make-cycle`: point the last pair of `x` back at `x` itself
 * and return `x`, closing the chain into a loop.
 */
public fun makeCycle(x: VPair): VPair {
    lastPair(x).setCdr(x)
    return x
}
