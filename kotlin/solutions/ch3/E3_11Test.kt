// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.11

package sicp.ch3.exercises

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_11Test :
    FunSpec({
        test("the book's session: deposit 40 gives 90, withdraw 60 gives 30") {
            val acc = makeBankAccount(50L)
            acc.deposit(40L) shouldBe 90L
            acc.withdraw(60L) shouldBe Either.Right(30L)
        }

        test("a second account keeps its own state") {
            val acc = makeBankAccount(50L)
            val acc2 = makeBankAccount(100L)
            acc2.withdraw(30L) shouldBe Either.Right(70L)
            acc.withdraw(80L) shouldBe Either.Left(WithdrawError.InsufficientFunds)
            acc.deposit(40L) shouldBe 90L
            acc2.withdraw(70L) shouldBe Either.Right(0L)
        }

        test("the two accounts are distinct objects but share the method bodies") {
            val acc = makeBankAccount(50L)
            val acc2 = makeBankAccount(100L)
            (acc === acc2) shouldBe false
            (acc === acc) shouldBe true
            acc2::class.java shouldBe acc::class.java
        }
    })
