// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.7

package sicp.ch1.exercises

import kotlin.math.abs

private fun improve(
    guess: Double,
    x: Double,
): Double = (guess + x / guess) / 2.0

private fun goodEnoughAbsolute(
    guess: Double,
    x: Double,
): Boolean = abs(guess * guess - x) < 0.001

/** The program of section 1.1.7 with its absolute-tolerance end test. */
public tailrec fun sqrtAbsolute(
    guess: Double = 1.0,
    x: Double,
): Double = if (goodEnoughAbsolute(guess, x)) guess else sqrtAbsolute(improve(guess, x), x)

/** A bounded probe: [steps] improvements with no end test applied. */
public fun improveSteps(
    x: Double,
    steps: Int,
): Double {
    var guess = 1.0
    repeat(steps) { guess = improve(guess, x) }
    return guess
}

/**
 * The design answer: stop when the change from one guess to the next is a
 * very small fraction of the guess itself.
 */
public tailrec fun ex_1_07(
    x: Double,
    guess: Double = 1.0,
    prior: Double = 0.0,
): Double = if (abs(guess - prior) < 0.001 * guess) guess else ex_1_07(x, improve(guess, x), guess)
