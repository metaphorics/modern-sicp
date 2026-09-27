// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.4

package sicp.ch3.exercises

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_04Test :
    FunSpec({
        test("seven consecutive wrong passwords are ordinary refusals") {
            val acc = makeAccountWithLockout(100L, "secret-password")
            repeat(7) { acc.withdraw("wrong", 1L) shouldBe Either.Left(AccountError.WrongPassword) }
        }

        test("the eighth consecutive wrong password calls the cops") {
            val acc = makeAccountWithLockout(100L, "secret-password")
            repeat(7) { acc.withdraw("wrong", 1L) }
            acc.withdraw("wrong", 1L) shouldBe Either.Left(AccountError.CallTheCops)
        }

        test("a correct password in between resets the streak") {
            val acc = makeAccountWithLockout(100L, "secret-password")
            repeat(6) { acc.withdraw("wrong", 1L) }
            acc.deposit("secret-password", 1L) shouldBe Either.Right(101L)
            repeat(7) { acc.withdraw("wrong", 1L) shouldBe Either.Left(AccountError.WrongPassword) }
            acc.withdraw("wrong", 1L) shouldBe Either.Left(AccountError.CallTheCops)
        }
    })
