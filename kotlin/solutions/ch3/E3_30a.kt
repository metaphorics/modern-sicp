// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.30a

package sicp.ch3.exercises

/**
 * The exercise 3.30a checker: one fresh [Simulation] and fourteen wires
 * per case, wired as a four-bit `Simulation.rippleCarryAdder`. The bits
 * of `a`, `b`, and `cIn` go onto the wires (wire 1 least significant,
 * as in figure 3.27), one `propagate` runs the schedule to exhaustion,
 * and the answer is the assembled result: the four sum wires as the low
 * four bits and the carry-out wire as bit five. For a correct adder
 * this equals the integer sum `a + b + cIn` for every input; the tests
 * enumerate the boundary cases and a seeded sweep.
 */
public fun rippleAdd(
    a: Int,
    b: Int,
    cIn: Int,
): Int {
    val sim = Simulation()
    val a1 = Wire()
    val a2 = Wire()
    val a3 = Wire()
    val a4 = Wire()
    val b1 = Wire()
    val b2 = Wire()
    val b3 = Wire()
    val b4 = Wire()
    val cInWire = Wire()
    val s1 = Wire()
    val s2 = Wire()
    val s3 = Wire()
    val s4 = Wire()
    val cOut = Wire()
    sim.rippleCarryAdder(a1, a2, a3, a4, b1, b2, b3, b4, cInWire, s1, s2, s3, s4, cOut)
    a1.setSignal((a and 1).toLong())
    a2.setSignal(((a shr 1) and 1).toLong())
    a3.setSignal(((a shr 2) and 1).toLong())
    a4.setSignal(((a shr 3) and 1).toLong())
    b1.setSignal((b and 1).toLong())
    b2.setSignal(((b shr 1) and 1).toLong())
    b3.setSignal(((b shr 2) and 1).toLong())
    b4.setSignal(((b shr 3) and 1).toLong())
    cInWire.setSignal(cIn.toLong())
    sim.propagate()
    val sumBits = s1.getSignal() + 2 * s2.getSignal() + 4 * s3.getSignal() + 8 * s4.getSignal()
    return (sumBits + 16 * cOut.getSignal()).toInt()
}
