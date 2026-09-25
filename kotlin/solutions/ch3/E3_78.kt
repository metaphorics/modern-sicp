// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.78

package sicp.ch3.exercises

import sicp.runtime.LStream

/**
 * The book's `solve-2nd` for `y'' = a y' + b y`: two integrals in a
 * loop. The acceleration stream adds the scaled velocity `a dy` and the
 * scaled position `b y`; it feeds the inner integral producing `dy` from
 * [dy0], whose output feeds the outer integral producing `y` from [y0].
 * The loop closes because both integrals take delayed integrands and
 * `dy` is tied before either tail can force it.
 */
public fun solve2nd(
    a: Double,
    b: Double,
    dt: Double,
    y0: Double,
    dy0: Double,
): LStream<Double> {
    var dy: LStream<Double>? = null
    val y: LStream<Double> = integral(lazy { checkNotNull(dy) { "dy not yet tied" } }, y0, dt)
    dy =
        integral(
            lazy {
                val ddy = checkNotNull(dy) { "dy not yet tied" }
                addStreams(scaleStream(ddy, a), scaleStream(y, b))
            },
            dy0,
            dt,
        )
    return y
}
