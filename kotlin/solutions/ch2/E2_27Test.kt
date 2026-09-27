// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.27

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_27Test :
    FunSpec({
        test("reverseTree reverses only the top level: ((3 4) (1 2))") {
            toBookString(reverseTree(bookPairTree())) shouldBe "((3 4) (1 2))"
        }
        test("deepReverse reverses at every level: ((4 3) (2 1))") {
            toBookString(deepReverse(bookPairTree())) shouldBe "((4 3) (2 1))"
        }
        test("deepReverse of (1 (2 3 4)) is ((4 3 2) 1)") {
            toBookString(deepReverse(tree(leaf(1L), tree(leaf(2L), leaf(3L), leaf(4L))))) shouldBe "((4 3 2) 1)"
        }
        test("ex_2_27 prints ((4 3) (2 1))") {
            ex_2_27() shouldBe "((4 3) (2 1))"
        }
    })
