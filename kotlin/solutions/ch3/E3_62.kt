// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.62

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.streamHead

/**
 * Exercise 3.62: division of power series. S1/S2 = (1/c) S1 * (1/(S2/c))
 * where c is S2's constant term: scaling the denominator by 1/c makes
 * it a unit series, [invertUnitSeries] from exercise 3.61 takes its
 * reciprocal, [mulSeries] from exercise 3.60 multiplies, and one more
 * 1/c factor restores the constant. A zero constant term cannot be
 * scaled away, so it is rejected.
 */
public fun divSeries(
    s1: LStream<Rat>,
    s2: LStream<Rat>,
): LStream<Rat> {
    val c = s2.streamHead()
    require(c != null && c != Rat.ZERO) { "the denominator series must start with a nonzero constant term" }
    val inverseConstant = Rat.ONE / c
    val unitDenominator = streamMap({ it * inverseConstant }, s2)
    return streamMap({ it * inverseConstant }, mulSeries(s1, invertUnitSeries(unitDenominator)))
}

/** The series of tan x: sin x divided by cos x, exactly. */
public val tangentSeries: LStream<Rat> = divSeries(sinSeries, cosSeries)
