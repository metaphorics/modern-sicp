// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.27

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_27Test :
    FunSpec({
        test("reverseTree changes only the root's child order") {
            reverseTree(twoBranchTree()) shouldBe
                tree(tree(leaf(3L), leaf(4L)), tree(leaf(1L), leaf(2L)))
        }
        test("deepReverse reverses child order recursively") {
            deepReverse(twoBranchTree()) shouldBe
                tree(tree(leaf(4L), leaf(3L)), tree(leaf(2L), leaf(1L)))
        }
        test("deepReverse reverses each nested node's children") {
            val input = tree(leaf(1L), tree(leaf(2L), leaf(3L), leaf(4L)))
            deepReverse(input) shouldBe tree(tree(leaf(4L), leaf(3L), leaf(2L)), leaf(1L))
        }
        test("ex_2_27 returns the recursively reversed sample tree") {
            ex_2_27() shouldBe tree(tree(leaf(4L), leaf(3L)), tree(leaf(2L), leaf(1L)))
        }
    })
