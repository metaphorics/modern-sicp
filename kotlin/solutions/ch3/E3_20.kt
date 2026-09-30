// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.20

package sicp.ch3.exercises

import sicp.runtime.Datum

/**
 * A procedural pair of 3.3.1: two mutable slots captured by the selectors
 * and updates, demonstrating shared state through aliases.
 */
public interface PairProc {
    public fun first(): Datum

    public fun second(): Datum

    public fun setFirst(v: Datum)

    public fun setSecond(v: Datum)
}

public fun proceduralPair(
    x: Datum,
    y: Datum,
): PairProc {
    var firstSlot = x
    var secondSlot = y
    return object : PairProc {
        override fun first(): Datum = firstSlot

        override fun second(): Datum = secondSlot

        override fun setFirst(v: Datum) {
            firstSlot = v
        }

        override fun setSecond(v: Datum) {
            secondSlot = v
        }
    }
}
