// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.12

package sicp.ch2.exercises

// Interval, makeInterval, lowerBound, and upperBound are exercise 2.7's public declarations; width is exercise 2.9's.

public fun center(i: Interval): Double = (i.lowerBound + i.upperBound) / 2.0

/** [p] is a fraction of the center, so ten percent is `0.1`, matching [percent]'s own units. */
public fun makeCenterPercent(
    c: Double,
    p: Double,
): Interval {
    val w = c * p
    return makeInterval(c - w, c + w)
}

public fun percent(i: Interval): Double = width(i) / center(i)

public fun ex_2_12(): Interval = makeCenterPercent(50.0, 0.1)
