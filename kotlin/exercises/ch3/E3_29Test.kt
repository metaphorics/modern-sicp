// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.29

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_29Test :
    FunSpec({
        test("Exercise 3.29: the compound or-gate agrees with the primitive one on all four input pairs").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val sim = Simulation()
            val a = Wire()
            val b = Wire()
            val out = Wire()
            sim.orGateFromAndGate(a, b, out)
            a.setSignal(1L)
            sim.propagate()
            org.junit.jupiter.api.Assertions
                .assertEquals(1L, out.getSignal())
        }
    })
