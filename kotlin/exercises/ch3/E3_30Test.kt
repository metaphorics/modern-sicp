// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.30

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_30Test :
    FunSpec({
        test("Exercise 3.30: the 4-bit ripple-carry adder adds two nibbles and a carry in").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
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
            sim.rippleCarryAdder(a1, a2, a3, a4, b1, b2, b3, b4, cIn, s1, s2, s3, s4, cOut)
            org.junit.jupiter.api.Assertions
                .assertEquals(0L, s1.getSignal())
        }
    })
