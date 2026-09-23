// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.40

package sicp.ch1.exercises

private const val TOLERANCE = 0.00001
private const val DX = 0.00001

private fun fixedPoint(
    f: (Double) -> Double,
    firstGuess: Double,
): Double {
    tailrec fun tryGuess(guess: Double): Double {
        val next = f(guess)
        return if (kotlin.math.abs(guess - next) < TOLERANCE) next else tryGuess(next)
    }
    return tryGuess(firstGuess)
}

private fun deriv(g: (Double) -> Double): (Double) -> Double = { x -> (g(x + DX) - g(x)) / DX }

private fun newtonTransform(g: (Double) -> Double): (Double) -> Double = { x -> x - g(x) / deriv(g)(x) }

public fun newtonsMethod(
    g: (Double) -> Double,
    guess: Double,
): Double = fixedPoint(newtonTransform(g), guess)

/** `cubic(a, b, c)` computes `x -> x^3 + a*x^2 + b*x + c`, one argument at a time. */
public fun cubic(
    a: Double,
    b: Double,
    c: Double,
): (Double) -> Double = { x -> x * x * x + a * x * x + b * x + c }

public fun ex_1_40(): Double = newtonsMethod(cubic(0.0, 0.0, -8.0), 1.0)
