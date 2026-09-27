// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.21

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VInt
import sicp.runtime.vlist

public class E2_21Test :
    FunSpec({
        test("both squareList definitions turn (1 2 3 4) into (1 4 9 16)") {
            val items = vlist(VInt(1L), VInt(2L), VInt(3L), VInt(4L))
            squareList(items).toString() shouldBe "(1 4 9 16)"
            squareListViaMap(items).toString() shouldBe "(1 4 9 16)"
        }
        test("the two definitions agree on negatives") {
            val items = vlist(VInt(-10L), VInt(2L), VInt(-11L), VInt(17L))
            squareList(items).toString() shouldBe squareListViaMap(items).toString()
        }
        test("ex_2_21 prints (1 4 9 16)") {
            ex_2_21() shouldBe "(1 4 9 16)"
        }
    })
