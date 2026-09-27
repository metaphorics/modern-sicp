// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.46

package sicp.ch1.exercises

import kotlin.math.abs
import kotlin.math.cos

/**
 * `goodEnough` tests the *current* guess (the section 1.1.7 `sqrtIter` shape); `iter` returns
 * that guess, not one more `improve` past it, when it is already good enough.
 */
public fun iterativeImprove(
    goodEnough: (Double) -> Boolean,
    improve: (Double) -> Double,
): (Double) -> Double {
    tailrec fun iter(guess: Double): Double = if (goodEnough(guess)) guess else iter(improve(guess))
    return ::iter
}

private fun average(
    x: Double,
    y: Double,
): Double = (x + y) / 2.0

/** Section 1.1.7's sqrt, rewritten in terms of iterativeImprove. */
public fun sqrtViaIterativeImprove(x: Double): Double =
    iterativeImprove({ guess -> abs(guess * guess - x) < 0.001 }, { guess -> average(guess, x / guess) })(1.0)

private const val TOLERANCE = 0.00001

/** Section 1.3.3's fixedPoint, rewritten in terms of iterativeImprove. */
public fun fixedPointViaIterativeImprove(
    f: (Double) -> Double,
    firstGuess: Double,
): Double = iterativeImprove({ guess -> abs(guess - f(guess)) < TOLERANCE }, f)(firstGuess)

public fun ex_1_46(): Pair<Double, Double> = Pair(sqrtViaIterativeImprove(9.0), fixedPointViaIterativeImprove(::cos, 1.0))
