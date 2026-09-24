// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.31

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_31Test :
    FunSpec({
        test("treeMap with identity keeps the tree") {
            treeMap({ x -> x }, nestedTree()) shouldBe nestedTree()
        }
        test("squareTreeViaTreeMap agrees with squareTree") {
            squareTreeViaTreeMap(nestedTree()) shouldBe squareTree(nestedTree())
        }
        test("ex_2_31 prints (1 (4 (9 16) 25) (36 49))") {
            ex_2_31() shouldBe "(1 (4 (9 16) 25) (36 49))"
        }
    })
