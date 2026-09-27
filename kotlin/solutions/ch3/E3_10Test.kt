// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.10

package sicp.ch3.exercises

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_10Test :
    FunSpec({
        test("the book's let session: w1 = makeWithdrawLet(100L), then w1(50L) is 50") {
            val w1 = makeWithdrawLet(100L)
            w1(50L) shouldBe Either.Right(50L)
        }

        test("the two versions of make-withdraw create objects with the same behavior") {
            val fromParameter = makeWithdraw(100L)
            val fromLet = makeWithdrawLet(100L)
            for (amount in listOf(50L, 60L, 20L, 40L)) {
                fromParameter(amount) shouldBe fromLet(amount)
            }
        }

        test("two let-version objects keep separate cells") {
            val w1 = makeWithdrawLet(100L)
            val w2 = makeWithdrawLet(100L)
            w1(50L) shouldBe Either.Right(50L)
            w2(70L) shouldBe Either.Right(30L)
            w2(40L) shouldBe Either.Left(WithdrawError.InsufficientFunds)
            w1(40L) shouldBe Either.Right(10L)
        }
    })
