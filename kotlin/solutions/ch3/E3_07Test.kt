// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.7

package sicp.ch3.exercises

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_07Test :
    FunSpec({
        test("the joint account's new password withdraws from the original account") {
            val peterAcc = makeAccount(100L, "open-sesame")
            val paulAcc = makeJoint(peterAcc, "open-sesame", "rosebud")
            paulAcc.withdraw("rosebud", 40L) shouldBe Either.Right(60L)
        }

        test("the original password still keeps working on the original account") {
            val peterAcc = makeAccount(100L, "open-sesame")
            makeJoint(peterAcc, "open-sesame", "rosebud")
            peterAcc.withdraw("open-sesame", 10L) shouldBe Either.Right(90L)
        }

        test("a joint transaction is visible through the original name: one shared balance") {
            val peterAcc = makeAccount(100L, "open-sesame")
            val paulAcc = makeJoint(peterAcc, "open-sesame", "rosebud")
            paulAcc.deposit("rosebud", 50L) shouldBe Either.Right(150L)
            peterAcc.withdraw("open-sesame", 150L) shouldBe Either.Right(0L)
        }

        test("a joint built with the wrong original password never grants access") {
            val peterAcc = makeAccount(100L, "open-sesame")
            val badJoint = makeJoint(peterAcc, "wrong-password", "rosebud")
            badJoint.withdraw("rosebud", 10L) shouldBe Either.Left(AccountError.WrongPassword)
        }
    })
