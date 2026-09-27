// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.17

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VInt
import sicp.runtime.vlist

public class E2_17Test :
    FunSpec({
        test("lastPair of (23 72 149 34) is (34)") {
            lastPair(vlist(VInt(23L), VInt(72L), VInt(149L), VInt(34L))).toString() shouldBe "(34)"
        }
        test("lastPair of a one-element list is the list itself") {
            lastPair(vlist(VInt(9L))).toString() shouldBe "(9)"
        }
        test("ex_2_17 prints (34)") {
            ex_2_17() shouldBe "(34)"
        }
    })
