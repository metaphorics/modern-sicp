// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.36

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_36Test :
    FunSpec({
        test("accumulateN sums the book's columns to (22 26 30)") {
            ex_2_36() shouldBe listOf(22L, 26L, 30L)
        }
        test("accumulating a single sequence leaves it alone under addition with 0") {
            accumulateN({ x, acc -> x + acc }, 0L, listOf(listOf(1L, 2L))) shouldBe listOf(1L, 2L)
        }
    })
