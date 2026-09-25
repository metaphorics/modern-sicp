// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.78

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * The book's `solve-2nd` for the homogeneous second-order equation
 * `y'' = a y' + b y`: two integrals in a loop. The acceleration
 * `a dy + b y` feeds the inner integral producing `dy` from [dy0], whose
 * output feeds the outer integral producing `y` from [y0].
 */
public fun solve2nd(
    a: Double,
    b: Double,
    dt: Double,
    y0: Double,
    dy0: Double,
): LStream<Double> = throw PendingSolution()
