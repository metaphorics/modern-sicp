// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.12

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_12Test :
    FunSpec({
        test("row 4 of Pascal's triangle") {
            ex_1_12(4) shouldBe listOf(1L, 4L, 6L, 4L, 1L)
        }
        test("the first rows and the two edges of every row") {
            ex_1_12(0) shouldBe listOf(1L)
            ex_1_12(1) shouldBe listOf(1L, 1L)
            ex_1_12(2) shouldBe listOf(1L, 2L, 1L)
            (0..6).forEach { row ->
                pascal(row, 0) shouldBe 1L
                pascal(row, row) shouldBe 1L
            }
        }
    })
