// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.79

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * The general second-order solver: `y'' = f(y', y)`. The acceleration is
 * [f] applied to the current velocity and position, integrated twice by
 * the same loop of two delayed integrals as [solve2nd].
 */
public fun solve2ndGeneral(
    f: (Double, Double) -> Double,
    dt: Double,
    y0: Double,
    dy0: Double,
): LStream<Double> = throw PendingSolution()
