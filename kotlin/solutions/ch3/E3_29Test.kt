// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.29

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_29Test :
    FunSpec({
        test("the compound and the primitive or-gates agree on all four input pairs") {
            for (x in 0L..1L) {
                for (y in 0L..1L) {
                    val primitive = Simulation()
                    val pa = Wire()
                    val pb = Wire()
                    val pOut = Wire()
                    primitive.orGate(pa, pb, pOut)
                    pa.setSignal(x)
                    pb.setSignal(y)
                    primitive.propagate()

                    val compound = Simulation()
                    val ca = Wire()
                    val cb = Wire()
                    val cOut = Wire()
                    compound.orGateFromAndGate(ca, cb, cOut)
                    ca.setSignal(x)
                    cb.setSignal(y)
                    compound.propagate()

                    pOut.getSignal() shouldBe cOut.getSignal()
                    cOut.getSignal() shouldBe (if (x == 1L || y == 1L) 1L else 0L)
                }
            }
        }

        test("the compound gate's delay is 2 inverter-delays plus one and-gate-delay") {
            val sim = Simulation()
            val a = Wire()
            val b = Wire()
            val out = Wire()
            sim.orGateFromAndGate(a, b, out)
            // Settle the initial wiring before measuring, so no transient
            // print lands in the log.
            sim.propagate()
            sim.probe("out", out)
            a.setSignal(1L)
            sim.propagate()
            sim.probeLog
                .toList() shouldBe
                listOf(
                    "-- out = 0, current-time = 7",
                    "-- out = 1, current-time = 14",
                )
            out.getSignal() shouldBe 1L
        }
    })
