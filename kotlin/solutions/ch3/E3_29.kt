// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.29

package sicp.ch3.exercises

/**
 * The book's compound or-gate: or(a, b) = not(and(not(a), not(b))),
 * built from two inverters, one and-gate, and one final inverter over
 * the internal wires `notA`, `notB`, and `bothOff`. The slowest path
 * from an input change to the output runs one inverter (2), the
 * and-gate (3), and the final inverter (2): a delay of
 * 2 * [inverterDelay] + [andGateDelay], against the primitive or-gate's
 * single [orGateDelay] from exercise 3.28.
 */
public fun Simulation.orGateFromAndGate(
    a1: Wire,
    a2: Wire,
    output: Wire,
) {
    val notA = Wire()
    val notB = Wire()
    val bothOff = Wire()
    inverter(a1, notA)
    inverter(a2, notB)
    andGate(notA, notB, bothOff)
    inverter(bothOff, output)
}
