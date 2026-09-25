// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.61

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.streamHead
import sicp.runtime.streamTail

/**
 * Exercise 3.61: 1/S for a series S whose constant term is 1. Write
 * S = 1 + S_R; the reciprocal X satisfies X = 1 - S_R X, so the
 * constant term is 1 and the tail is (-S_R) multiplied by X itself,
 * with [mulSeries] from exercise 3.60 doing the multiplication.
 */
public fun invertUnitSeries(s: LStream<Rat>): LStream<Rat> {
    require(s.streamHead() == Rat.ONE) { "invert-unit-series needs a constant term of 1" }
    return consStream(Rat.ONE) {
        mulSeries(
            streamMap({ it * Rat.of(-1L) }, s.streamTail()),
            invertUnitSeries(s),
        )
    }
}
