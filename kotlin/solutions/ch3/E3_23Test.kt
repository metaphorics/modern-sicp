// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.23

package sicp.ch3.exercises

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe
import sicp.runtime.VInt
import sicp.runtime.VSym

public class E3_23Test :
    FunSpec({
        test("insert at both ends, then delete from both ends") {
            val d = makeDeque()
            d.rearInsert(VSym("a"))
            d.rearInsert(VSym("b"))
            d.frontInsert(VSym("z"))
            d.frontDeque() shouldBe VSym("z")
            d.rearDeque() shouldBe VSym("b")
            d.frontDelete() shouldBe Either.Right(VSym("z"))
            d.rearDelete() shouldBe Either.Right(VSym("b"))
            d.frontDelete() shouldBe Either.Right(VSym("a"))
            d.emptyDeque() shouldBe true
        }

        test("the empty deque fails at both ends with EmptyDeque") {
            val d = makeDeque()
            d.emptyDeque() shouldBe true
            d.frontDeque().shouldBeNull()
            d.rearDeque().shouldBeNull()
            d.frontDelete() shouldBe Either.Left(DequeError.EmptyDeque)
            d.rearDelete() shouldBe Either.Left(DequeError.EmptyDeque)
        }

        test("the deque is reusable after emptying, from either end") {
            val d = makeDeque()
            d.rearInsert(VSym("a"))
            d.rearDelete() shouldBe Either.Right(VSym("a"))
            d.frontInsert(VSym("b"))
            d.frontDelete() shouldBe Either.Right(VSym("b"))
            d.emptyDeque() shouldBe true
            d.rearInsert(VSym("c"))
            d.frontDeque() shouldBe VSym("c")
        }

        test("mixed traffic keeps both ends consistent") {
            val d = makeDeque()
            d.frontInsert(VInt(1))
            d.rearInsert(VInt(2))
            d.frontInsert(VInt(3))
            d.rearInsert(VInt(4))
            d.frontDeque() shouldBe VInt(3)
            d.rearDeque() shouldBe VInt(4)
            d.rearDelete() shouldBe Either.Right(VInt(4))
            d.frontDelete() shouldBe Either.Right(VInt(3))
            d.frontDelete() shouldBe Either.Right(VInt(1))
            d.rearDelete() shouldBe Either.Right(VInt(2))
        }
    })
