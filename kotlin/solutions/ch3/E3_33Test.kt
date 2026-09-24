// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.33

package sicp.ch3.exercises

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_33Test :
    FunSpec({
        test("the book's direction: a and b give c the average") {
            val a = Connector()
            val b = Connector()
            val c = Connector()
            averager(a, b, c)
            a.setValue(10L, User) shouldBe Either.Right(Unit)
            b.setValue(20L, User) shouldBe Either.Right(Unit)
            c.value() shouldBe 15L
        }

        test("the reverse direction: c and a give b") {
            val a = Connector()
            val b = Connector()
            val c = Connector()
            averager(a, b, c)
            c.setValue(9L, User) shouldBe Either.Right(Unit)
            a.setValue(8L, User) shouldBe Either.Right(Unit)
            b.value() shouldBe 10L
        }

        test("forgetting a retracts the derived values; user-set b stands") {
            val a = Connector()
            val b = Connector()
            val c = Connector()
            averager(a, b, c)
            a.setValue(10L, User)
            b.setValue(20L, User)
            a.forgetValue(User)
            a.hasValue() shouldBe false
            b.hasValue() shouldBe true
            b.value() shouldBe 20L
            c.hasValue() shouldBe false
        }

        test("an odd sum truncates: the documented Long behavior") {
            val a = Connector()
            val b = Connector()
            val c = Connector()
            averager(a, b, c)
            a.setValue(7L, User)
            b.setValue(10L, User)
            c.value() shouldBe 8L
        }
    })
