// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.28

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_28Test :
    FunSpec({
        test("Exercise 3.28: the primitive or-gate drives its output one orGateDelay after an input changes").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val sim = Simulation()
            val a = Wire()
            val b = Wire()
            val out = Wire()
            sim.orGate(a, b, out)
            a.setSignal(1L)
            sim.propagate()
            org.junit.jupiter.api.Assertions
                .assertEquals(1L, out.getSignal())
        }
    })
