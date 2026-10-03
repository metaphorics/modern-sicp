// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_16

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.guest.GValue

public class E5_16Test :
    FunSpec({
        test("Exercise 5.16: the traced run lists every executed instruction, ending at the taken branch") {
            tracedGcdTrace() shouldBe
                listOf(
                    "(test (op =) (reg b) (const 0))",
                    "(branch (label gcd-done))",
                    "(assign t (op rem) (reg a) (reg b))",
                    "(assign a (reg b))",
                    "(assign b (reg t))",
                    "(goto (label test-b))",
                    "(test (op =) (reg b) (const 0))",
                    "(branch (label gcd-done))",
                    "(assign t (op rem) (reg a) (reg b))",
                    "(assign a (reg b))",
                    "(assign b (reg t))",
                    "(goto (label test-b))",
                    "(test (op =) (reg b) (const 0))",
                    "(branch (label gcd-done))",
                    "(assign t (op rem) (reg a) (reg b))",
                    "(assign a (reg b))",
                    "(assign b (reg t))",
                    "(goto (label test-b))",
                    "(test (op =) (reg b) (const 0))",
                    "(branch (label gcd-done))",
                    "(assign t (op rem) (reg a) (reg b))",
                    "(assign a (reg b))",
                    "(assign b (reg t))",
                    "(goto (label test-b))",
                    "(test (op =) (reg b) (const 0))",
                    "(branch (label gcd-done))",
                )
        }
        test("Exercise 5.16: the switch off leaves no trace lines") {
            val lines =
                TracingMachine(
                    setOf("a", "b", "t"),
                    machineArithmetic,
                    gcdController,
                    mapOf("a" to GValue.VLong(206), "b" to GValue.VLong(40)),
                ).apply { traceOn = false }.run()
            lines.filter { it.isNotBlank() } shouldBe emptyList()
        }
    })
