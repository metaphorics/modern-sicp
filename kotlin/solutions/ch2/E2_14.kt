// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.14

package sicp.ch2.exercises

// addInterval, mulInterval, divInterval, makeInterval are exercise 2.7's; makeCenterPercent, percent are exercise 2.12's.

public fun par1(
    r1: Interval,
    r2: Interval,
): Interval = divInterval(mulInterval(r1, r2), addInterval(r1, r2))

public fun par2(
    r1: Interval,
    r2: Interval,
): Interval {
    val one = makeInterval(1.0, 1.0)
    return divInterval(one, addInterval(divInterval(one, r1), divInterval(one, r2)))
}

/**
 * `a / a` is exactly 1 for any nonzero real number `a` ranges over, so a
 * correct system would report zero tolerance; naive interval arithmetic
 * cannot know the numerator's `a` and the denominator's `a` are the same
 * quantity, and reports roughly double `a`'s own tolerance instead. `a / b`,
 * built from two genuinely independent intervals, reports roughly the sum
 * of their tolerances, matching exercise 2.13's approximation.
 */
public fun ex_2_14(): Pair<Double, Double> {
    val a = makeCenterPercent(100.0, 0.05)
    val b = makeCenterPercent(200.0, 0.1)
    return percent(divInterval(a, a)) to percent(divInterval(a, b))
}
