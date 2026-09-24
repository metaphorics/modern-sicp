// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.30

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/** Builds a fresh four-bit adder over fourteen wires and returns it
 * with its wires, ready for a test to set and read bits. */
private class RippleFixture {
    val sim = Simulation()
    val a1 = Wire()
    val a2 = Wire()
    val a3 = Wire()
    val a4 = Wire()
    val b1 = Wire()
    val b2 = Wire()
    val b3 = Wire()
    val b4 = Wire()
    val cIn = Wire()
    val s1 = Wire()
    val s2 = Wire()
    val s3 = Wire()
    val s4 = Wire()
    val cOut = Wire()

    init {
        sim.rippleCarryAdder(a1, a2, a3, a4, b1, b2, b3, b4, cIn, s1, s2, s3, s4, cOut)
    }

    fun bits(
        a: Int,
        b: Int,
        c: Int,
    ) {
        a1.setSignal((a and 1).toLong())
        a2.setSignal(((a shr 1) and 1).toLong())
        a3.setSignal(((a shr 2) and 1).toLong())
        a4.setSignal(((a shr 3) and 1).toLong())
        b1.setSignal((b and 1).toLong())
        b2.setSignal(((b shr 1) and 1).toLong())
        b3.setSignal(((b shr 2) and 1).toLong())
        b4.setSignal(((b shr 3) and 1).toLong())
        cIn.setSignal(c.toLong())
    }

    fun sum(): Int = s1.getSignal().toInt() + 2 * s2.getSignal().toInt() + 4 * s3.getSignal().toInt() + 8 * s4.getSignal().toInt()
}

public class E3_30Test :
    FunSpec({
        test("0110 + 0011 = 1001 with no carry out") {
            val f = RippleFixture()
            f.bits(6, 3, 0)
            f.sim.propagate()
            f.sum() shouldBe 9
            f.cOut.getSignal() shouldBe 0L
        }

        test("1111 + 0001 ripples the carry all the way out") {
            val f = RippleFixture()
            f.bits(15, 1, 0)
            f.sim.propagate()
            f.sum() shouldBe 0
            f.cOut.getSignal() shouldBe 1L
        }

        test("a carry in counts as the stage-1 addition's third input") {
            val f = RippleFixture()
            f.bits(0, 0, 1)
            f.sim.propagate()
            f.sum() shouldBe 1
            f.cOut.getSignal() shouldBe 0L
        }

        test("the carry ripples stage by stage, not all at once") {
            val f = RippleFixture()
            f.sim.probe("cOut", f.cOut)
            f.bits(15, 1, 0)
            f.sim.propagate()
            val lines = f.sim.probeLog.toList()
            // The registration line, then the settled carry out: the last
            // flip on cOut closes the ripple.
            lines.first() shouldBe "-- cOut = 0, current-time = 0"
            lines.last() shouldBe "-- cOut = 1, current-time = 64"
        }

        test("flipping only the carry in restages the ripple through the sums") {
            val f = RippleFixture()
            f.bits(7, 0, 0)
            f.sim.propagate()
            f.sum() shouldBe 7
            for ((name, wire) in listOf("s1" to f.s1, "s2" to f.s2, "s3" to f.s3, "s4" to f.s4)) {
                f.sim.probe(name, wire)
            }
            val registered = f.sim.probeLog.size
            f.cIn.setSignal(1L)
            f.sim.propagate()
            f.sum() shouldBe 8
            val times =
                f.sim.probeLog
                    .drop(registered)
                    .map { line -> line.substringAfter("current-time = ").toInt() }
            // One settled print per sum wire, each strictly later than the
            // stage before it: the carry is propagating, not jumping.
            times.sorted() shouldBe times
            times.toSet().size shouldBe times.size
        }
    })
