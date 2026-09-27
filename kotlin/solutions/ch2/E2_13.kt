// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.13

package sicp.ch2.exercises

// makeCenterPercent and percent are exercise 2.12's public declarations; mulInterval is exercise 2.7's.

/**
 * For positive centers and small fractional tolerances `p1` and `p2`, the
 * product's exact percentage tolerance is `(p1 + p2) / (1 + p1 * p2)`; the
 * `p1 * p2` term is second order and negligible when `p1` and `p2` are
 * small, leaving the simple approximation `p1 + p2`.
 */
public fun approxProductPercent(
    p1: Double,
    p2: Double,
): Double = p1 + p2

public fun ex_2_13(): Pair<Double, Double> {
    val x = makeCenterPercent(100.0, 0.01)
    val y = makeCenterPercent(50.0, 0.02)
    val exact = percent(mulInterval(x, y))
    return exact to approxProductPercent(0.01, 0.02)
}
