// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.13

package sicp.ch3.exercises

import sicp.runtime.PairCell

/** Close the final link of [x] back onto its first pair. */
public fun makeCycle(x: PairCell): PairCell {
    lastPair(x).second = x
    return x
}
