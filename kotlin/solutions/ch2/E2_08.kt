// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.8

package sicp.ch2.exercises

// Interval, makeInterval, lowerBound, and upperBound are exercise 2.7's public declarations, reused here.

/**
 * The smallest the difference could be is the smallest `x` minus the
 * largest `y`; the largest is the largest `x` minus the smallest `y`.
 */
public fun subInterval(
    x: Interval,
    y: Interval,
): Interval = makeInterval(x.lowerBound - y.upperBound, x.upperBound - y.lowerBound)

public fun ex_2_08(): Interval = subInterval(makeInterval(6.0, 8.0), makeInterval(3.0, 5.0))
