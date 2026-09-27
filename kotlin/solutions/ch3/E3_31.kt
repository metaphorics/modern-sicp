// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.31

package sicp.ch3.exercises

/**
 * Exercise 3.31's evidence: the book's half-adder session, up to the
 * point the book starts changing signals. `Wire.addAction` runs each
 * newly added action once at registration, so each probe prints its
 * wire's initial state the moment it is attached, and every gate action
 * the wiring installs schedules its wire's initial output (the
 * settling events the first `propagate` would otherwise miss). The
 * answer is the probe log before any signal changes and before any
 * `propagate` call.
 */
public fun registrationProbeLog(): List<String> {
    val sim = Simulation()
    val input1 = Wire()
    val input2 = Wire()
    val sum = Wire()
    val carry = Wire()
    sim.probe("sum", sum)
    sim.probe("carry", carry)
    sim.halfAdder(input1, input2, sum, carry)
    return sim.probeLog.toList()
}
