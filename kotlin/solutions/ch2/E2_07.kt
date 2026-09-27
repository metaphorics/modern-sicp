// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.7

package sicp.ch2.exercises

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

public fun ex_2_07(): Interval = makeInterval(6.12, 7.48)
