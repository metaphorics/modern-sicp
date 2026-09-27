// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.31

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 3.31: `Wire.addAction` runs a newly added action once,
 * immediately. Explain why this initialization is necessary: trace the
 * book's half-adder session and say how the system's response would
 * differ if `addAction` had merely appended the action without running
 * it.
 *
 * This edition makes the trace observable: [registrationProbeLog]
 * wires the book's half-adder under two probes on a fresh simulation
 * and answers the probe log exactly as it stands before any signal
 * changes and before any `propagate` call.
 */
public fun registrationProbeLog(): List<String> = throw PendingSolution()
