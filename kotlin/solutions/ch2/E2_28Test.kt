// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.28

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_28Test :
    FunSpec({
        test("fringe lists every leaf in left-to-right order") {
            ex_2_28() shouldBe listOf(1L, 2L, 3L, 4L)
        }
        test("reusing a subtree repeats its leaves in place") {
            fringe(tree(twoBranchTree(), twoBranchTree())) shouldBe
                listOf(1L, 2L, 3L, 4L, 1L, 2L, 3L, 4L)
        }
        test("a leaf has itself as its fringe") {
            fringe(leaf(9L)) shouldBe listOf(9L)
        }
    })
