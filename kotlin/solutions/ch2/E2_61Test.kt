// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.61

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_61Test :
    FunSpec({
        test("adjoinSetOrdered inserts in the middle to keep increasing order") {
            adjoinSetOrdered(4L, listOf(1L, 3L, 6L, 10L)) shouldBe listOf(1L, 3L, 4L, 6L, 10L)
        }
        test("adjoinSetOrdered of an already-present element returns the set unchanged") {
            adjoinSetOrdered(6L, listOf(1L, 3L, 6L, 10L)) shouldBe listOf(1L, 3L, 6L, 10L)
        }
        test("adjoinSetOrdered can insert at either end") {
            adjoinSetOrdered(0L, listOf(1L, 3L)) shouldBe listOf(0L, 1L, 3L)
            adjoinSetOrdered(9L, listOf(1L, 3L)) shouldBe listOf(1L, 3L, 9L)
        }
        test("ex_2_61 inserts 4 into [1, 3, 6, 10]") {
            ex_2_61() shouldBe listOf(1L, 3L, 4L, 6L, 10L)
        }
    })
