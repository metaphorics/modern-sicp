// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.35

package sicp.ch1.exercises

import kotlin.math.abs

private const val TOLERANCE = 0.00001

private fun fixedPoint(
    f: (Double) -> Double,
    firstGuess: Double,
): Double {
    tailrec fun tryGuess(guess: Double): Double {
        val next = f(guess)
        return if (abs(guess - next) < TOLERANCE) next else tryGuess(next)
    }
    return tryGuess(firstGuess)
}

/**
 * The golden ratio `phi` satisfies `phi^2 = phi + 1`, so `phi = 1 + 1/phi`: phi is a fixed
 * point of `x -> 1 + 1/x`.
 */
public fun ex_1_35(): Double = fixedPoint({ x -> 1.0 + 1.0 / x }, 1.0)
