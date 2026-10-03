// SPDX-License-Identifier: GPL-3.0-only
// Corpus case core/04-sqrt-newton: Newton iteration and block structure
fun improve(guess: Double, x: Double): Double = (guess + x / guess) / 2.0

fun goodEnough(guess: Double, x: Double): Boolean {
    val diff = guess * guess - x
    val magnitude = if (diff < 0.0) -diff else diff
    return magnitude < 0.001
}

fun sqrtIter(guess: Double, x: Double): Double =
    if (goodEnough(guess, x)) guess else sqrtIter(improve(guess, x), x)

fun sqrt(x: Double): Double = sqrtIter(1.0, x)

fun main() {
    println(sqrt(2.0))
}
