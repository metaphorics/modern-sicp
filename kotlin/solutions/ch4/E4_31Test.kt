// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.31

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_31Test :
    FunSpec({
        test("Exercise 4.31: the declared mix counts 5") {
            annotatedMixedTranscript() shouldBe "(1 5 5 4 30 30)\n5\n"
        }

        test("Exercise 4.31: all lazy-memo counts 4") {
            annotatedAllMemoTranscript() shouldBe "(1 5 5 4 30 30)\n4\n"
        }

        test("Exercise 4.31: a lazy parameter skips its dangerous argument") {
            lazyParamSkipsTranscript() shouldBe "7\n"
        }

        test("Exercise 4.31: a strict parameter evaluates at the call") {
            strictParamEagerTranscript() shouldBe "Error: division by zero\n"
        }
    })
