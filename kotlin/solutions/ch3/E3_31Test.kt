// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.31

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_31Test :
    FunSpec({
        test("the probes print their wires' initial state at registration, before anything runs") {
            val log = registrationProbeLog()
            log shouldBe
                listOf(
                    "-- sum = 0, current-time = 0",
                    "-- carry = 0, current-time = 0",
                )
        }

        test("the immediate run leaves settling events on the agenda, not just prints") {
            // Rebuild the registration by hand to inspect the agenda: the
            // half-adder's gates each scheduled their initial output.
            val sim = Simulation()
            val input1 = Wire()
            val input2 = Wire()
            val sum = Wire()
            val carry = Wire()
            sim.halfAdder(input1, input2, sum, carry)
            sim.agenda.emptyAgenda() shouldBe false
            sim.propagate()
            // The settling events resolved to the initial state: quiet.
            sum.getSignal() shouldBe 0L
            carry.getSignal() shouldBe 0L
        }
    })
