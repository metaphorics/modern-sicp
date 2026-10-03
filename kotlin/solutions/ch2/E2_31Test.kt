// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.31

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_31Test :
    FunSpec({
        test("treeMap with identity preserves every node and leaf") {
            treeMap({ value -> value }, nestedTree()) shouldBe nestedTree()
        }
        test("mapping a square agrees with the direct tree traversal") {
            squareTreeViaTreeMap(nestedTree()) shouldBe squareTree(nestedTree())
        }
        test("ex_2_31 returns the typed squared tree") {
            ex_2_31() shouldBe squareTree(nestedTree())
        }
    })
