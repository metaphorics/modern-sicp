// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.44

package sicp.ch1.exercises

private const val DX = 0.00001

public fun smooth(f: (Double) -> Double): (Double) -> Double = { x -> (f(x - DX) + f(x) + f(x + DX)) / 3.0 }

// repeated is exercise 1.43's public procedure, reused here from the same package.

/**
 * The n-fold smoothed function: `repeated` applied to the *smoothing transformation*
 * `::smooth`, not to `f` itself, so it needs `repeated` instantiated at
 * `T = (Double) -> Double` rather than `T = Double`.
 */
public fun nFoldSmooth(
    f: (Double) -> Double,
    n: Long,
): (Double) -> Double = repeated(::smooth, n)(f)

private fun square(x: Double): Double = x * x

public fun ex_1_44(): Pair<Double, Double> = Pair(smooth(::square)(2.0), nFoldSmooth(::square, 5L)(2.0))
