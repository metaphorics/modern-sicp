// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.34

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_34Test :
    FunSpec({
        test("Exercise 4.34: dotted and proper lazy pairs print in their shapes") {
            lazyPairPrintTranscript() shouldBe "(1 . 2)\n(1 2)\n"
        }

        test("Exercise 4.34: the infinite list prints prefix plus ellipsis") {
            onesBudgetPrintTranscript() shouldBe "(1 1 1 1 1 1 1 1 1 1 ...)\n"
        }

        test("Exercise 4.34: car forces only the demanded element") {
            carOfOnesTranscript() shouldBe "1\n"
        }

        test("Exercise 4.34: nested lazy pairs print recursively") {
            nestedLazyPrintTranscript() shouldBe "((1) 2)\n"
        }
    })
