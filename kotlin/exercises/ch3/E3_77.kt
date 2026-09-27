// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.77

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * The `integers-starting-from` style `integral`: the first element is
 * [initialValue] alone, and [delayedIntegrand] is forced only past it,
 * which is what lets a feedback loop close. A finite integrand ends the
 * stream.
 */
public fun integralAlt(
    delayedIntegrand: Lazy<LStream<Double>>,
    initialValue: Double,
    dt: Double,
): LStream<Double> = throw PendingSolution()

/**
 * The 3.5.4 `solve` wired through [integralAlt]: `dy/dt = f(y)` as a
 * feedback loop whose delayed integrand is tied back to the solution
 * stream itself.
 */
public fun solveAlt(
    f: (Double) -> Double,
    y0: Double,
    dt: Double,
): LStream<Double> = throw PendingSolution()
