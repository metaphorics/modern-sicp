// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.34

package sicp.ch3.exercises

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_34Test :
    FunSpec({
        test("the flaw observed: setting b leaves a unknown") {
            val a = Connector()
            val b = Connector()
            louisSquarer(a, b)
            b.setValue(25L, User) shouldBe Either.Right(Unit)
            a.hasValue() shouldBe false
        }

        test("the forward direction still squares") {
            val a = Connector()
            val b = Connector()
            louisSquarer(a, b)
            a.setValue(3L, User) shouldBe Either.Right(Unit)
            b.value() shouldBe 9L
        }

        test("the device holds the stale square: b = 25 contradicts it") {
            val a = Connector()
            val b = Connector()
            louisSquarer(a, b)
            a.setValue(3L, User)
            b.setValue(25L, User) shouldBe
                Either.Left(ConstraintError.Contradiction(9L, 25L))
        }
    })
