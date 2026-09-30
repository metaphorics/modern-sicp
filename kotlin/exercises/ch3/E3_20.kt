// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.20

package sicp.ch3.exercises

import sicp.runtime.Datum
import sicp.runtime.PendingSolution

/**
 * Exercise 3.20 represents a pair with an object whose methods close over
 * two mutable slots. After `alias = x` and `alias.setFirst(Whole(17L))`,
 * `x.first()` returns 17 because both names refer to the same object.
 * A fresh `proceduralPair` has independent captured slots.
 */
public interface PairProc {
    public fun first(): Datum

    public fun second(): Datum

    public fun setFirst(v: Datum)

    public fun setSecond(v: Datum)
}

/** A pair object whose two slots are mutable locals captured by its methods. */
public fun proceduralPair(
    x: Datum,
    y: Datum,
): PairProc = throw PendingSolution()
