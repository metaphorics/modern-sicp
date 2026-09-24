// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.2.3, frames as the repository of local state

package sicp.ch3.examples

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/**
 * The book's alternate `make-withdraw` for exercise 3.10: the state
 * variable is a local `var` instead of a parameter. In Scheme the local
 * binding is a `let`, sugar for a procedure call that builds a frame of
 * its own; a Kotlin local `var` binds in the frame the function call
 * already built, so the returned lambda captures exactly one cell, the
 * same cell the parameter version captures.
 */
public fun makeWithdrawLet(initialAmount: Long): (Long) -> Either<WithdrawError, Long> {
    var balance = initialAmount
    return { amount ->
        if (amount > balance) {
            Either.Left(WithdrawError.InsufficientFunds)
        } else {
            balance -= amount
            Either.Right(balance)
        }
    }
}

public class S3_2_3FramesAsRepositoryOfLocalStateTest :
    FunSpec({
        test("the book's session: w1 = makeWithdraw(100), then w1(50) is 50") {
            val w1 = makeWithdraw(100L)
            w1(50L) shouldBe Either.Right(50L)
        }

        test("the let version: w1 = makeWithdrawLet(100), then w1(50) is 50") {
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

        test("a second let-version object keeps its own cell") {
            val w1 = makeWithdrawLet(100L)
            val w2 = makeWithdrawLet(100L)
            w1(50L) shouldBe Either.Right(50L)
            w2(70L) shouldBe Either.Right(30L)
            w2(40L) shouldBe Either.Left(WithdrawError.InsufficientFunds)
            w1(40L) shouldBe Either.Right(10L)
        }
    })
