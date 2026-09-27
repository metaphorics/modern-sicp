// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.1

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_01Test :
    FunSpec({
        test("the book's session: makeAccumulator(5), then 10 and 10 give 15 and 25") {
            val a = makeAccumulator(5L)
            a(10L) shouldBe 15L
            a(10L) shouldBe 25L
        }

        test("two accumulators keep independent sums") {
            val a = makeAccumulator(0L)
            val b = makeAccumulator(100L)
            a(1L) shouldBe 1L
            b(1L) shouldBe 101L
            a(1L) shouldBe 2L
        }
    })
