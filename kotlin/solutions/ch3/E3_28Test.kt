// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.28

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_28Test :
    FunSpec({
        test("the or-gate truth table over all four input pairs") {
            for (x in 0L..1L) {
                for (y in 0L..1L) {
                    val sim = Simulation()
                    val a = Wire()
                    val b = Wire()
                    val out = Wire()
                    sim.orGate(a, b, out)
                    a.setSignal(x)
                    b.setSignal(y)
                    sim.propagate()
                    out.getSignal() shouldBe (if (x == 1L || y == 1L) 1L else 0L)
                }
            }
        }

        test("the output settles exactly one orGateDelay after an input changes") {
            val sim = Simulation()
            val a = Wire()
            val b = Wire()
            val out = Wire()
            sim.orGate(a, b, out)
            sim.probe("out", out)
            a.setSignal(1L)
            sim.propagate()
            sim.probeLog
                .toList() shouldBe
                listOf(
                    "-- out = 0, current-time = 0",
                    "-- out = 1, current-time = 5",
                )
            out.getSignal() shouldBe 1L
        }

        test("a second input change to an already-true output prints nothing") {
            val sim = Simulation()
            val a = Wire()
            val b = Wire()
            val out = Wire()
            sim.orGate(a, b, out)
            sim.probe("out", out)
            a.setSignal(1L)
            b.setSignal(1L)
            sim.propagate()
            sim.probeLog
                .toList() shouldBe
                listOf(
                    "-- out = 0, current-time = 0",
                    "-- out = 1, current-time = 5",
                )
        }
    })
