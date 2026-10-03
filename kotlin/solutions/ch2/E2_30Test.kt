// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.30

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_30Test :
    FunSpec({
        test("squareTree squares each leaf and keeps every branch in place") {
            val expected = tree(leaf(1L), tree(leaf(4L), tree(leaf(9L), leaf(16L)), leaf(25L)), tree(leaf(36L), leaf(49L)))
            squareTree(nestedTree()) shouldBe expected
        }
        test("the map-based traversal agrees with the direct traversal") {
            squareTreeViaMap(nestedTree()) shouldBe squareTree(nestedTree())
        }
        test("ex_2_30 returns the typed result tree") {
            ex_2_30() shouldBe squareTree(nestedTree())
        }
    })
