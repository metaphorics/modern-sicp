// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.62

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_62Test :
    FunSpec({
        test("unionSetOrdered merges two ordered lists, deduplicating shared elements") {
            unionSetOrdered(listOf(1L, 3L, 6L, 10L), listOf(3L, 6L, 9L)) shouldBe listOf(1L, 3L, 6L, 9L, 10L)
        }
        test("unionSetOrdered of disjoint sets interleaves them in order") {
            unionSetOrdered(listOf(1L, 5L), listOf(2L, 4L)) shouldBe listOf(1L, 2L, 4L, 5L)
        }
        test("unionSetOrdered with an empty set returns the other set") {
            unionSetOrdered(emptyList(), listOf(1L, 2L)) shouldBe listOf(1L, 2L)
            unionSetOrdered(listOf(1L, 2L), emptyList()) shouldBe listOf(1L, 2L)
        }
        test("ex_2_62 unions [1,3,6,10] and [3,6,9]") {
            ex_2_62() shouldBe listOf(1L, 3L, 6L, 9L, 10L)
        }
    })
