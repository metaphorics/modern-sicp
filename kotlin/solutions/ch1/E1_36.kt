// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.36

package sicp.ch1.exercises

import kotlin.math.abs
import kotlin.math.ln

private const val TOLERANCE = 0.00001

/** [fixedPoint], recording every approximation instead of only the last: `result.size` is the step count. */
public fun fixedPointTraced(
    f: (Double) -> Double,
    firstGuess: Double,
): List<Double> {
    tailrec fun trace(
        guess: Double,
        seen: List<Double>,
    ): List<Double> {
        val next = f(guess)
        val soFar = seen + next
        return if (abs(guess - next) < TOLERANCE) soFar else trace(next, soFar)
    }
    return trace(firstGuess, emptyList())
}

private fun average(
    x: Double,
    y: Double,
): Double = (x + y) / 2.0

private fun dampXToTheX(f: (Double) -> Double): (Double) -> Double = { x -> average(x, f(x)) }

private fun xToTheX(x: Double): Double = ln(1000.0) / ln(x)

public fun ex_1_36(): Pair<Int, Int> {
    val withoutDamping = fixedPointTraced(::xToTheX, 2.0).size
    val withDamping = fixedPointTraced(dampXToTheX(::xToTheX), 2.0).size
    return Pair(withoutDamping, withDamping)
}
