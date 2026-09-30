// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.1: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_01Test :
    FunSpec({
        test("Exercise 4.1: the kernel walk evaluates operands left to right") {
            leftToRightTranscript() shouldBe "(1 . 2)\n1\n2\n"
        }

        test("Exercise 4.1: the variant walk evaluates operands right to left") {
            rightToLeftTranscript() shouldBe "(1 . 2)\n2\n1\n"
        }
    })
