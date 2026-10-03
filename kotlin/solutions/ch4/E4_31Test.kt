// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.31: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E431Test :
    FunSpec({
        test("Exercise 4.31: the mixed discipline counts 5 with every value in place") {
            annotatedMixedTranscript() shouldBe "[1, 5, 5, 4, 30, 30]\n5\n"
        }

        test("Exercise 4.31: the all-memo discipline counts 4 on the same values") {
            annotatedAllMemoTranscript() shouldBe "[1, 5, 5, 4, 30, 30]\n4\n"
        }

        test("Exercise 4.31: a delayed parameter is never evaluated unless demanded") {
            lazyParamSkipsTranscript() shouldBe "7\n"
        }

        test("Exercise 4.31: a strict parameter is evaluated at the call") {
            strictParamEagerTranscript() shouldBe "Error: DivisionByZero\n"
        }
    })
