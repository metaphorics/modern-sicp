// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.23

package sicp.ch3.exercises

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe
import sicp.runtime.Symbol
import sicp.runtime.Whole

public class E3_23Test :
    FunSpec({
        test("insert at both ends, then delete from both ends") {
            val d = makeDeque()
            d.rearInsert(Symbol("a"))
            d.rearInsert(Symbol("b"))
            d.frontInsert(Symbol("z"))
            d.frontDeque() shouldBe Symbol("z")
            d.rearDeque() shouldBe Symbol("b")
            d.frontDelete() shouldBe Either.Right(Symbol("z"))
            d.rearDelete() shouldBe Either.Right(Symbol("b"))
            d.frontDelete() shouldBe Either.Right(Symbol("a"))
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
            d.rearInsert(Symbol("a"))
            d.rearDelete() shouldBe Either.Right(Symbol("a"))
            d.frontInsert(Symbol("b"))
            d.frontDelete() shouldBe Either.Right(Symbol("b"))
            d.emptyDeque() shouldBe true
            d.rearInsert(Symbol("c"))
            d.frontDeque() shouldBe Symbol("c")
        }

        test("mixed traffic keeps both ends consistent") {
            val d = makeDeque()
            d.frontInsert(Whole(1))
            d.rearInsert(Whole(2))
            d.frontInsert(Whole(3))
            d.rearInsert(Whole(4))
            d.frontDeque() shouldBe Whole(3)
            d.rearDeque() shouldBe Whole(4)
            d.rearDelete() shouldBe Either.Right(Whole(4))
            d.frontDelete() shouldBe Either.Right(Whole(3))
            d.frontDelete() shouldBe Either.Right(Whole(1))
            d.rearDelete() shouldBe Either.Right(Whole(2))
        }
    })
