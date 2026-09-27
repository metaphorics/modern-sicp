// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.37

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_37Test :
    FunSpec({
        test("dotProduct of (1 2 3 4) and (4 5 6 6) is 56") {
            dotProduct(listOf(1L, 2L, 3L, 4L), listOf(4L, 5L, 6L, 6L)) shouldBe 56L
        }
        test("matrixStarVector with (1 1 1 1) gives the row sums (10 21 30)") {
            ex_2_37() shouldBe listOf(10L, 21L, 30L)
        }
        test("transpose of the book's matrix has columns as rows") {
            transpose(exerciseMatrix()) shouldBe
                listOf(
                    listOf(1L, 4L, 6L),
                    listOf(2L, 5L, 7L),
                    listOf(3L, 6L, 8L),
                    listOf(4L, 6L, 9L),
                )
        }
        test("matrixStarMatrix with the transposed matrix gives the Gram rows") {
            matrixStarMatrix(exerciseMatrix(), transpose(exerciseMatrix()))[0] shouldBe listOf(30L, 56L, 80L)
        }
    })
