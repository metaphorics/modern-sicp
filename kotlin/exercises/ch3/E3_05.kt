// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.5

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

private const val XORSHIFT_MULTIPLIER: ULong = 0x2545F4914F6CDD1DUL

/** One step of the seeded xorshift64* generator; see book/ch3/3.1.texi section 3.1.2 for the shared explanation. */
private fun randUpdate(x: ULong): ULong {
    var y = x
    y = y xor (y shr 12)
    y = y xor (y shl 25)
    y = y xor (y shr 27)
    return y * XORSHIFT_MULTIPLIER
}

/** A closure over one word of hidden state: the section's `rand`, reused here as the exercise's own generator. */
public fun makeRand(seed: ULong): () -> ULong {
    var x = seed
    return {
        x = randUpdate(x)
        x
    }
}

/** The section's `monte-carlo`, reused here since exercises compile on their own. */
public fun monteCarlo(
    trials: Int,
    experiment: () -> Boolean,
): Double {
    tailrec fun iter(
        remaining: Int,
        passed: Int,
    ): Double =
        when {
            remaining == 0 -> passed.toDouble() / trials
            experiment() -> iter(remaining - 1, passed + 1)
            else -> iter(remaining - 1, passed)
        }
    return iter(trials, 0)
}

/**
 * Exercise 3.5: Monte Carlo integration is a method of estimating
 * definite integrals by means of Monte Carlo simulation. Consider
 * computing the area of a region of space described by a predicate
 * `p(x, y)` that is true for points in the region and false for points
 * not in the region. To estimate the area, choose a rectangle that
 * contains the region, pick points at random from within the rectangle,
 * and test `p` at each one; the fraction of points that fall in the
 * region, times the rectangle's area, estimates the integral.
 *
 * Implement Monte Carlo integration as `estimateIntegral`, taking a
 * predicate `p`, the rectangle's bounds `x1`, `x2`, `y1`, `y2`, a
 * random-word generator `next`, and the number of trials. Use the same
 * `monteCarlo` used above to estimate pi. `randomInRange` returns a
 * value chosen at random from `low` (inclusive) to `high` (exclusive),
 * built from the generator's raw word.
 *
 * The scaffold's `randomInRange` always returns `low`, and
 * `estimateIntegral` always returns the rectangle's full area.
 */
public fun randomInRange(
    low: Double,
    high: Double,
    next: () -> ULong,
): Double = throw PendingSolution()

public fun estimateIntegral(
    trials: Int,
    x1: Double,
    x2: Double,
    y1: Double,
    y2: Double,
    next: () -> ULong,
    p: (Double, Double) -> Boolean,
): Double = throw PendingSolution()

/** The book's closing ask: estimate pi from `estimateIntegral` measured over a unit circle. */
public fun estimatePiViaIntegral(
    trials: Int,
    next: () -> ULong,
): Double = estimateIntegral(trials, -1.0, 1.0, -1.0, 1.0, next) { x, y -> x * x + y * y <= 1.0 }
