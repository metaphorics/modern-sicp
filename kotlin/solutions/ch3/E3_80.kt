// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.80

package sicp.ch3.exercises

import sicp.runtime.LStream

/**
 * The series RLC circuit of Figure 3.37. [rlc] takes the resistance
 * [r], inductance [l], capacitance [c], and the time step [dt], and
 * returns a function from the initial state `vC0`, `iL0` to the pair of
 * state streams. The two coupled integrals close the loop together:
 * the capacitor voltage integrates `- iL / c` from `vC0`, the inductor
 * current integrates `(vC - r iL) / l` from `iL0`, and each integrand
 * is tied before any tail can force it.
 */
public fun rlc(
    r: Double,
    l: Double,
    c: Double,
    dt: Double,
): (Double, Double) -> Pair<LStream<Double>, LStream<Double>> =
    { vC0, iL0 ->
        var iL: LStream<Double>? = null
        val vC: LStream<Double> =
            integral(lazy { scaleStream(checkNotNull(iL) { "iL not yet tied" }, -1.0 / c) }, vC0, dt)
        val tied: LStream<Double> =
            integral(
                lazy {
                    val current = checkNotNull(iL) { "iL not yet tied" }
                    zipStream(vC, current) { v, i -> (v - r * i) / l }
                },
                iL0,
                dt,
            )
        iL = tied
        Pair(vC, tied)
    }
