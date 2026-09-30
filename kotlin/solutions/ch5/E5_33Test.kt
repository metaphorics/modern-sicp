// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_33

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_33Test :
    FunSpec({
        test("Exercise 5.33: the two factorial compilations answer alike") {
            factorialComparison().takeLast(2) shouldBe
                listOf(
                    "the two runs answer alike: true",
                    "the alternative's pending operand uses deeper stack: true",
                )
        }
    })
