// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_04

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_04Test :
    FunSpec({
        test("Exercise 5.4: the two exponentiation machines") {
            exptMachineRuns() shouldBe
                listOf(
                    "1024",
                    "243",
                    "1024",
                    "243",
                )
        }
        test("Exercise 5.4: the recursive machine stacks exactly n frames") {
            exptRecursiveMaxDepth(b = 2, n = 10) shouldBe 10
            exptRecursiveMaxDepth(b = 3, n = 5) shouldBe 5
        }
    })
