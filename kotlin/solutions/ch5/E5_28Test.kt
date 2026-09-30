// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_28

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_28Test :
    FunSpec({
        test("with the naive sequence evaluation both factorials demand space that grows with n") {
            val lines = nonTailRecursiveMeasurements()
            lines.takeLast(2) shouldBe
                listOf(
                    "recursive maximum depth now grows with n: true",
                    "iterative maximum depth now grows with n: true",
                )
        }
        test("the non-tail controller preserves the factorial answer") {
            nonTailAnswersAgree() shouldBe true
        }
    })
