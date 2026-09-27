// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.30

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_30Test :
    FunSpec({
        test("squareTree squares every leaf of (1 (2 (3 4) 5) (6 7))") {
            toBookString(squareTree(nestedTree())) shouldBe "(1 (4 (9 16) 25) (36 49))"
        }
        test("the map-based squareTree agrees with the direct one") {
            squareTreeViaMap(nestedTree()) shouldBe squareTree(nestedTree())
        }
        test("ex_2_30 prints (1 (4 (9 16) 25) (36 49))") {
            ex_2_30() shouldBe "(1 (4 (9 16) 25) (36 49))"
        }
    })
