// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.54

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.VSym
import sicp.runtime.vlist

public class E2_54Test :
    FunSpec({
        test("myEqual is true for two lists of the same symbols in the same order") {
            val a = vlist(VSym("this"), VSym("is"), VSym("a"), VSym("list"))
            val b = vlist(VSym("this"), VSym("is"), VSym("a"), VSym("list"))
            myEqual(a, b) shouldBe true
        }
        test("myEqual is false when the nesting shape differs") {
            val a = vlist(VSym("this"), VSym("is"), VSym("a"), VSym("list"))
            val b = vlist(VSym("this"), vlist(VSym("is"), VSym("a")), VSym("list"))
            myEqual(a, b) shouldBe false
        }
        test("myEqual is true on two empty lists and false for different lengths") {
            myEqual(VNil, VNil) shouldBe true
            myEqual(vlist(VSym("a")), VNil) shouldBe false
        }
        test("myEqual is false between a symbol and a number, even with matching text") {
            myEqual(VSym("1"), VInt(1L)) shouldBe false
        }
        test("ex_2_54 matches the book's two worked examples") {
            ex_2_54() shouldBe listOf(true, false)
        }
    })
