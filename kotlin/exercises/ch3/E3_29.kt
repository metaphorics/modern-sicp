// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.29

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 3.29: build an or-gate as a compound digital logic device
 * from and-gates and inverters: or(a, b) = not(and(not(a), not(b))).
 * Wire two inverters from the inputs, feed their outputs through an
 * and-gate, and feed the and-gate's output through one more inverter to
 * the output wire. The delay of the compound gate is
 * 2 * inverterDelay + andGateDelay: the two input inverters run in
 * parallel, the and-gate adds its delay, and the final inverter adds
 * its own.
 *
 * The book reuses the name `or-gate` for this compound device, but
 * exercise 3.28's primitive `orGate` already occupies that name in this
 * package, so the compound constructor is `orGateFromAndGate`.
 */
public fun Simulation.orGateFromAndGate(
    a1: Wire,
    a2: Wire,
    output: Wire,
): Unit = throw PendingSolution()
