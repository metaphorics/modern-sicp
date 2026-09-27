// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.9

package sicp.ch2.exercises

// Interval, makeInterval, lowerBound, upperBound, addInterval, and mulInterval are exercise 2.7's public declarations.

public fun width(i: Interval): Double = (i.upperBound - i.lowerBound) / 2.0

public fun ex_2_09(): Pair<Double, Double> {
    val product1 = mulInterval(makeInterval(1.0, 3.0), makeInterval(1.0, 3.0))
    val product2 = mulInterval(makeInterval(10.0, 12.0), makeInterval(10.0, 12.0))
    return width(product1) to width(product2)
}
