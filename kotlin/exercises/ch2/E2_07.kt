// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.7

package sicp.ch2.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 2.7: Alyssa's program is incomplete because she has not
 * specified the implementation of the interval abstraction. Here is a
 * definition of the interval constructor, `makeInterval`, given below.
 * Define selectors `lowerBound` and `upperBound` to complete the
 * implementation. The statement lives in the section 2.1 chapter text.
 *
 * A Kotlin data class exposes its constructor arguments as named
 * properties automatically, so `lowerBound` and `upperBound` need no
 * separate body once `Interval` itself is declared this way; that
 * automatic exposure is the whole exercise once the host language has
 * real records.
 *
 * `addInterval`, `mulInterval`, and `divInterval` are the main text's own
 * operations (section 2.1.4, given before this exercise), reproduced here
 * because this exercises module cannot see the examples module; exercises
 * 2.9 through 2.16 reuse this file's declarations from the same package,
 * the way exercise 1.43a reused exercise 1.42's `compose`.
 *
 * The scaffold returns `makeInterval(6.12, 7.48)`.
 */
public data class Interval(
    val lowerBound: Double,
    val upperBound: Double,
)

public fun makeInterval(
    a: Double,
    b: Double,
): Interval = Interval(a, b)

public fun addInterval(
    x: Interval,
    y: Interval,
): Interval = makeInterval(x.lowerBound + y.lowerBound, x.upperBound + y.upperBound)

public fun mulInterval(
    x: Interval,
    y: Interval,
): Interval {
    val p1 = x.lowerBound * y.lowerBound
    val p2 = x.lowerBound * y.upperBound
    val p3 = x.upperBound * y.lowerBound
    val p4 = x.upperBound * y.upperBound
    return makeInterval(minOf(p1, p2, p3, p4), maxOf(p1, p2, p3, p4))
}

public fun divInterval(
    x: Interval,
    y: Interval,
): Interval = mulInterval(x, makeInterval(1.0 / y.upperBound, 1.0 / y.lowerBound))

public fun ex_2_07(): Interval = throw PendingSolution()
