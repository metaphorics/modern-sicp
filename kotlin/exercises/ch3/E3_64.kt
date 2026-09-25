// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.64

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * Exercise 3.64: examine [s] until two successive elements differ in
 * absolute value by less than [tolerance], and return the second of
 * the two elements.
 */
public fun streamLimit(
    s: LStream<Double>,
    tolerance: Double,
): Double = throw PendingSolution()

/** Square roots up to a tolerance: the limit of the guess stream. */
public fun sqrtByLimit(
    x: Double,
    tolerance: Double,
): Double = throw PendingSolution()
