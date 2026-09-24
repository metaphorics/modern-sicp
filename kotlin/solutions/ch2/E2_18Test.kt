// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.18

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VInt
import sicp.runtime.vlist

public class E2_18Test :
    FunSpec({
        test("reverseList of (1 4 9 16 25) is (25 16 9 4 1)") {
            reverseList(vlist(VInt(1L), VInt(4L), VInt(9L), VInt(16L), VInt(25L))).toString() shouldBe "(25 16 9 4 1)"
        }
        test("reversing twice returns the original") {
            val items = vlist(VInt(1L), VInt(4L), VInt(9L), VInt(16L), VInt(25L))
            reverseList(reverseList(items)).toString() shouldBe items.toString()
        }
        test("ex_2_18 prints (25 16 9 4 1)") {
            ex_2_18() shouldBe "(25 16 9 4 1)"
        }
    })
