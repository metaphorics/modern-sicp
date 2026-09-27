// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.28

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_28Test :
    FunSpec({
        test("fringe of ((1 2) (3 4)) is the four leaves") {
            ex_2_28() shouldBe listOf(1L, 2L, 3L, 4L)
        }
        test("fringe of (list x x) repeats the four leaves") {
            fringe(tree(bookPairTree(), bookPairTree())) shouldBe listOf(1L, 2L, 3L, 4L, 1L, 2L, 3L, 4L)
        }
        test("fringe of a single leaf is the leaf") {
            fringe(leaf(9L)) shouldBe listOf(9L)
        }
    })
