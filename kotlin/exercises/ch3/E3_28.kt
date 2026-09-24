// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.28

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/** The book's or-gate-delay. */
public val orGateDelay: Long = 5L

/** The book's logical-or. */
public fun logicalOr(
    a: Long,
    b: Long,
): Long = throw PendingSolution()

/**
 * Exercise 3.28: define an or-gate as a primitive function box, similar
 * to the section's `Simulation.andGate`. The constructor attaches one
 * action to each input wire; the action computes the logical or of the
 * two input signals and schedules the output to take that value one
 * [orGateDelay] later.
 *
 * The other gate constructors are members of `Simulation`; this
 * exercise's deliverable lives in its own file, so it is an extension
 * on `Simulation` with the same call shape.
 */
public fun Simulation.orGate(
    a1: Wire,
    a2: Wire,
    output: Wire,
): Unit = throw PendingSolution()
