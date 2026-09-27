// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.11

package sicp.ch2.exercises

// Interval, makeInterval, lowerBound, upperBound, and mulInterval are exercise 2.7's public declarations.

/**
 * Nine cases on the signs of the four endpoints. Eight of the nine need
 * only the two products at the relevant corners; the last, where both
 * intervals span zero, falls back to all four (Ben's "more than two").
 */
public fun mulIntervalCases(
    x: Interval,
    y: Interval,
): Interval {
    val xl = x.lowerBound
    val xu = x.upperBound
    val yl = y.lowerBound
    val yu = y.upperBound
    return when {
        xl >= 0.0 && yl >= 0.0 -> makeInterval(xl * yl, xu * yu)
        xl >= 0.0 && yu <= 0.0 -> makeInterval(xu * yl, xl * yu)
        xl >= 0.0 -> makeInterval(xu * yl, xu * yu)
        xu <= 0.0 && yl >= 0.0 -> makeInterval(xl * yu, xu * yl)
        xu <= 0.0 && yu <= 0.0 -> makeInterval(xu * yu, xl * yl)
        xu <= 0.0 -> makeInterval(xl * yu, xl * yl)
        yl >= 0.0 -> makeInterval(xl * yu, xu * yu)
        yu <= 0.0 -> makeInterval(xu * yl, xl * yl)
        else -> makeInterval(minOf(xl * yu, xu * yl), maxOf(xl * yl, xu * yu))
    }
}

public fun ex_2_11(): Interval = mulIntervalCases(makeInterval(-2.0, 3.0), makeInterval(-1.0, 2.0))
