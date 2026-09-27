// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.77

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream

/**
 * The `integers-starting-from` style `integral`: the first element is
 * the initial value alone, and the integrand is forced only past it,
 * which is what lets a feedback loop close. Each tail steps the
 * accumulated value forward and re-delays the remaining integrand. A
 * finite integrand ends the stream, the book's `stream-null?` branch.
 */
public fun integralAlt(
    delayedIntegrand: Lazy<LStream<Double>>,
    initialValue: Double,
    dt: Double,
): LStream<Double> =
    consStream(initialValue) {
        val integrand = delayedIntegrand.value
        when (integrand) {
            is LStream.Empty -> LStream.Empty
            is LStream.Cons -> integralAlt(lazy { integrand.tail }, initialValue + dt * integrand.head, dt)
        }
    }

/**
 * The 3.5.4 `solve` wired through [integralAlt]: `dy/dt = f(y)` as a
 * feedback loop whose delayed integrand is tied back to the solution
 * stream `y` itself inside the lazy value.
 */
public fun solveAlt(
    f: (Double) -> Double,
    y0: Double,
    dt: Double,
): LStream<Double> {
    var dy: LStream<Double>? = null
    val y: LStream<Double> = integralAlt(lazy { checkNotNull(dy) { "dy not yet tied" } }, y0, dt)
    dy = streamMap(f, y)
    return y
}
