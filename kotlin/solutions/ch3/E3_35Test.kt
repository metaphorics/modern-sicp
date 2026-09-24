// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.35

package sicp.ch3.exercises

import arrow.core.Either
import io.kotest.assertions.throwables.shouldThrow
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_35Test :
    FunSpec({
        test("b infers a through the square root") {
            val a = Connector()
            val b = Connector()
            squarer(a, b)
            b.setValue(25L, User) shouldBe Either.Right(Unit)
            a.value() shouldBe 5L
        }

        test("a infers b through the square") {
            val a = Connector()
            val b = Connector()
            squarer(a, b)
            a.setValue(7L, User) shouldBe Either.Right(Unit)
            b.value() shouldBe 49L
        }

        test("forgetting one side releases both, then re-derives") {
            val a = Connector()
            val b = Connector()
            squarer(a, b)
            a.setValue(7L, User)
            a.forgetValue(User)
            a.hasValue() shouldBe false
            b.hasValue() shouldBe false
            b.setValue(36L, User)
            a.value() shouldBe 6L
        }

        test("negative b raises the book's square-less-than-0 error") {
            val a = Connector()
            val b = Connector()
            squarer(a, b)
            shouldThrow<IllegalStateException> {
                b.setValue(-4L, User)
            }.message shouldBe "square less than 0: SQUARER -4"
        }
    })
