// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.3

package sicp.ch3.exercises

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_03Test :
    FunSpec({
        test("the book's session: withdraw 40 with the right password, then a wrong deposit password") {
            val acc = makeAccount(100L, "secret-password")
            acc.withdraw("secret-password", 40L) shouldBe Either.Right(60L)
            acc.deposit("some-other-password", 50L) shouldBe Either.Left(AccountError.WrongPassword)
        }

        test("a correct password over the balance raises InsufficientFunds, not WrongPassword") {
            val acc = makeAccount(100L, "secret-password")
            acc.withdraw("secret-password", 200L) shouldBe Either.Left(AccountError.InsufficientFunds)
        }

        test("two accounts never share a password or a balance") {
            val a = makeAccount(100L, "alpha")
            val b = makeAccount(100L, "beta")
            a.withdraw("beta", 10L) shouldBe Either.Left(AccountError.WrongPassword)
            b.withdraw("beta", 10L) shouldBe Either.Right(90L)
            a.withdraw("alpha", 10L) shouldBe Either.Right(90L)
        }
    })
