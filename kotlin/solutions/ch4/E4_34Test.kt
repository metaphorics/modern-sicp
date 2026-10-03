// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.34: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E434Test :
    FunSpec({
        test("Exercise 4.34: a proper list prints whole") {
            lazyProperPrintTranscript() shouldBe "(1 2)\n"
        }

        test("Exercise 4.34: the infinite list prints ten elements then the ellipsis") {
            onesBudgetPrintTranscript() shouldBe "(1 1 1 1 1 1 1 1 1 1 ...)\n"
        }

        test("Exercise 4.34: the head answers without walking the list") {
            carOfOnesTranscript() shouldBe "1\n"
        }

        test("Exercise 4.34: nested lazy pairs print recursively") {
            nestedLazyPrintTranscript() shouldBe "((1) (2))\n"
        }
    })
