// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.80

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * The series RLC circuit of Figure 3.37: [rlc] takes the resistance
 * [r], inductance [l], capacitance [c], and the time step [dt], and
 * returns a function from the initial state `vC0`, `iL0` to the pair of
 * state streams. The capacitor voltage integrates `- iL / c` from
 * `vC0`, the inductor current integrates `(vC - r iL) / l` from `iL0`,
 * and the two integrals close the loop together. For the statement's
 * circuit: `rlc(1.0, 1.0, 0.2, 0.1)(10.0, 0.0)`.
 */
public fun rlc(
    r: Double,
    l: Double,
    c: Double,
    dt: Double,
): (Double, Double) -> Pair<LStream<Double>, LStream<Double>> = throw PendingSolution()
