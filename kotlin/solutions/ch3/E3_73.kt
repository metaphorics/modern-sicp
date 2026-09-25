// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.73

package sicp.ch3.exercises

import sicp.runtime.LStream

/**
 * The RC circuit of Figure 3.33 as a signal-flow network: the resistor
 * branch scales the current by [r] and reaches the output at once, the
 * capacitor branch scales the current by `1 / [c]` and passes an
 * `integral` seeded with the initial capacitor voltage. The returned
 * function takes the current stream `i` and the initial voltage `v0` and
 * produces the voltage stream `v = v0 + (1/c) integral(i dt) + r i`.
 */
public fun rc(
    r: Double,
    c: Double,
    dt: Double,
): (LStream<Double>, Double) -> LStream<Double> =
    { i, v0 -> addStreams(scaleStream(i, r), integral(lazy { scaleStream(i, 1.0 / c) }, v0, dt)) }
