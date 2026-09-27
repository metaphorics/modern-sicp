// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_16

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch5.arithOperations
import sicp.ch5.getRegisterContents
import sicp.ch5.setRegisterContents
import sicp.runtime.VInt

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
                machineRun {
                    val machine = TracingMachine(listOf("a", "b", "t"), arithOperations)
                    machine.install(gcdController)
                    machine.setRegisterContents("a", VInt(206))
                    machine.setRegisterContents("b", VInt(40))
                    machine.traceOn = false
                    machine.start()
                    machine.transcript.toString().lines()
                }
            lines.filter { it.isNotBlank() } shouldBe emptyList()
        }
    })
