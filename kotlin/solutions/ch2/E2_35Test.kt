// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.35

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_35Test :
    FunSpec({
        test("countLeavesAccumulate counts the four leaves of (1 (2 (3 4)))") {
            ex_2_35() shouldBe 4L
        }
        test("countLeavesAccumulate of (list x x) doubles the count, the book's 8") {
            val x = tree(leaf(1L), tree(leaf(2L), leaf(3L), leaf(4L)))
            countLeavesAccumulate(tree(x, x)) shouldBe 8L
        }
        test("an empty node has no leaves") {
            countLeavesAccumulate(Tree.Node(emptyList())) shouldBe 0L
        }
    })
