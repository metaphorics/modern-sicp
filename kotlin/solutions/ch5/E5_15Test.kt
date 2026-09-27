// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_15

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_15Test :
    FunSpec({
        test("Exercise 5.15: the counting machine totals every executed instruction") {
            gcdInstructionCounts() shouldBe
                listOf(
                    "gcd(206, 40): 26 instructions",
                    "factorial(5): 49 instructions",
                )
        }
    })
