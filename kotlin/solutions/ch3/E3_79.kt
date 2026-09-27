// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.79

package sicp.ch3.exercises

import sicp.runtime.LStream

/**
 * The general second-order solver: `y'' = f(y', y)`. The acceleration is
 * [f] applied to the current velocity and position; the same loop of two
 * delayed integrals as [solve2nd] integrates it, so any second-order
 * equation is a one-argument change.
 */
public fun solve2ndGeneral(
    f: (Double, Double) -> Double,
    dt: Double,
    y0: Double,
    dy0: Double,
): LStream<Double> {
    var dy: LStream<Double>? = null
    val y: LStream<Double> = integral(lazy { checkNotNull(dy) { "dy not yet tied" } }, y0, dt)
    dy =
        integral(
            lazy {
                val current = checkNotNull(dy) { "dy not yet tied" }
                zipStream(current, y) { d, yy -> f(d, yy) }
            },
            dy0,
            dt,
        )
    return y
}
