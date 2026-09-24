// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.22

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VInt
import sicp.runtime.vlist

public class E2_22Test :
    FunSpec({
        test("the first attempt answers (16 9 4 1), the reverse of the desired order") {
            squareListIter(vlist(VInt(1L), VInt(2L), VInt(3L), VInt(4L))).toString() shouldBe "(16 9 4 1)"
        }
        test("the swapped attempt builds an improper chain of pairs, not a list") {
            squareListIterSwapped(vlist(VInt(1L), VInt(2L), VInt(3L), VInt(4L))).toString() shouldBe
                "((((() . 1) . 4) . 9) . 16)"
        }
        test("ex_2_22 shows Louis's reversed answer") {
            ex_2_22() shouldBe "(16 9 4 1)"
        }
    })
