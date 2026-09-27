// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.73

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * Model the RC circuit of Figure 3.33: a resistor of resistance [r] and
 * a capacitor of capacitance [c] in series, sampled every [dt]. The
 * returned function takes the current stream `i` and the initial
 * capacitor voltage `v0` and produces the voltage stream
 * `v = v0 + (1/c) integral(i dt) + r i`, the resistor branch scaled and
 * added to the integral branch at once. For the statement's circuit,
 * `rc1 = rc(5.0, 1.0, 0.5)` takes a current stream and an initial
 * voltage and yields the output voltages.
 */
public fun rc(
    r: Double,
    c: Double,
    dt: Double,
): (LStream<Double>, Double) -> LStream<Double> = throw PendingSolution()
