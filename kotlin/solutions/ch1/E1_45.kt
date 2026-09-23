// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.45

package sicp.ch1.exercises

import kotlin.math.abs
import kotlin.math.floor
import kotlin.math.ln
import kotlin.math.pow

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

private fun average(
    x: Double,
    y: Double,
): Double = (x + y) / 2.0

public fun averageDamp(f: (Double) -> Double): (Double) -> Double = { x -> average(x, f(x)) }

// repeated is exercise 1.43's public procedure, reused here from the same package.

/**
 * Experimentally, a fixed-point search for `y -> x / y^(n-1)` needs `floor(log2(n))` average
 * damps to converge: one for square roots (`n = 2`), two starting at fourth roots (`n = 4`),
 * three starting at eighth roots (`n = 8`), and so on, the same doubling boundary
 * `fastExpt` crosses in section 1.2.4.
 */
public fun nthRoot(
    x: Double,
    n: Long,
): Double {
    val damps = floor(ln(n.toDouble()) / ln(2.0)).toLong()
    val baseGuess = { y: Double -> x / y.pow((n - 1L).toDouble()) }
    val dampedGuess = repeated(::averageDamp, damps)(baseGuess)
    return fixedPoint(dampedGuess, 1.0)
}

public fun ex_1_45(): Pair<Double, Double> = Pair(nthRoot(16.0, 4L), nthRoot(256.0, 8L))
