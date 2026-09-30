// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.54

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Empty
import sicp.runtime.Symbol
import sicp.runtime.Whole
import sicp.runtime.datumList

public class E2_54Test :
    FunSpec({
        test("myEqual accepts independently allocated trees with the same contents") {
            val first = datumList(Symbol("this"), Symbol("is"), Symbol("a"), Symbol("list"))
            val second = datumList(Symbol("this"), Symbol("is"), Symbol("a"), Symbol("list"))
            myEqual(first, second) shouldBe true
        }
        test("myEqual rejects a different nesting shape") {
            val flat = datumList(Symbol("this"), Symbol("is"), Symbol("a"), Symbol("list"))
            val nested = datumList(Symbol("this"), datumList(Symbol("is"), Symbol("a")), Symbol("list"))
            myEqual(flat, nested) shouldBe false
        }
        test("myEqual compares empty and nonempty shapes consistently") {
            myEqual(Empty, Empty) shouldBe true
            myEqual(datumList(Symbol("a")), Empty) shouldBe false
        }
        test("myEqual distinguishes a symbol from a whole number") {
            myEqual(Symbol("1"), Whole(1L)) shouldBe false
        }
        test("ex_2_54 returns equal-shape and unequal-shape results") {
            ex_2_54() shouldBe listOf(true, false)
        }
    })
