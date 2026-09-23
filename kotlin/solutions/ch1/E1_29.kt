// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.29

package sicp.ch1.exercises

private fun cube(x: Double): Double = x * x * x

/** Simpson's Rule: `y_k`'s coefficient is 1 at the endpoints, 4 at odd `k`, 2 at even interior `k`. */
private fun simpsonCoefficient(
    k: Long,
    n: Long,
): Double =
    when {
        k == 0L || k == n -> 1.0
        k % 2L == 1L -> 4.0
        else -> 2.0
    }

public fun simpson(
    f: (Double) -> Double,
    a: Double,
    b: Double,
    n: Long,
): Double {
    val h = (b - a) / n

    fun y(k: Long): Double = f(a + k * h)
    var total = 0.0
    for (k in 0..n) total += simpsonCoefficient(k, n) * y(k)
    return total * h / 3.0
}

public fun ex_1_29(): Pair<Double, Double> = Pair(simpson(::cube, 0.0, 1.0, 100L), simpson(::cube, 0.0, 1.0, 1000L))
