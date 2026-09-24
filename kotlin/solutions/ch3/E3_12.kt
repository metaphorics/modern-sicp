// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.12

package sicp.ch3.exercises

import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.Value
import sicp.runtime.cons
import sicp.runtime.setCdr

/**
 * The book's `append` of 2.2.1: a fresh list built by consing each
 * element of `x` onto `y`; no pair of `x` is touched.
 */
public fun append(
    x: Value,
    y: Value,
): Value =
    if (x !is VPair) {
        y
    } else {
        cons(x.car, append(x.cdr, y))
    }

/** The book's `last-pair`: the final pair of a nonempty chain. */
public fun lastPair(x: VPair): VPair = if (x.cdr is VNil) x else lastPair(x.cdr as VPair)

/**
 * The book's `append!`: point the last pair of `x` at `y`, splicing the
 * two chains together and returning `x` itself.
 */
public fun appendBang(
    x: VPair,
    y: VPair,
): VPair {
    lastPair(x).setCdr(y)
    return x
}
