// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.28

package sicp.ch3.exercises

/** The book's or-gate-delay. */
public val orGateDelay: Long = 5L

/**
 * The book's logical-or: the conversion the or-gate's action applies to
 * the two input signals.
 */
public fun logicalOr(
    a: Long,
    b: Long,
): Long = if (a == 1L || b == 1L) 1L else 0L

/**
 * The book's or-gate as a primitive function box, built exactly like the
 * section's `Simulation.andGate`: one action serves both inputs, and
 * when either input changes it schedules the output to take the logical
 * or of the two signals one [orGateDelay] later. It is an extension on
 * `Simulation`, with the same call shape as the `Simulation` members,
 * because this exercise's code lives in its own file and may not
 * redeclare the class's members.
 */
public fun Simulation.orGate(
    a1: Wire,
    a2: Wire,
    output: Wire,
) {
    val orAction = {
        val newValue = logicalOr(a1.getSignal(), a2.getSignal())
        afterDelay(orGateDelay) { output.setSignal(newValue) }
    }
    a1.addAction(orAction)
    a2.addAction(orAction)
}
