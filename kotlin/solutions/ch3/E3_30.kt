// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.30

package sicp.ch3.exercises

/**
 * The book's ripple-carry adder at this edition's fixed width of four
 * bits: four [Simulation.fullAdder] stages chained exactly as figure
 * 3.27 wires them, stage 1 taking the external `cIn` and stage 4
 * producing `cOut`. The carries between stages are local wires named
 * for the figure's `C_1`, `C_2`, `C_3`; each stage's carry-in wire is
 * visible in the wiring calls, with no hidden rewiring.
 */
public fun Simulation.rippleCarryAdder(
    a1: Wire,
    a2: Wire,
    a3: Wire,
    a4: Wire,
    b1: Wire,
    b2: Wire,
    b3: Wire,
    b4: Wire,
    cIn: Wire,
    s1: Wire,
    s2: Wire,
    s3: Wire,
    s4: Wire,
    cOut: Wire,
) {
    val c1 = Wire()
    val c2 = Wire()
    val c3 = Wire()
    fullAdder(a1, b1, cIn, s1, c1)
    fullAdder(a2, b2, c1, s2, c2)
    fullAdder(a3, b3, c2, s3, c3)
    fullAdder(a4, b4, c3, s4, cOut)
}
