// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.37

package sicp.ch3.exercises

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_37Test :
    FunSpec({
        test("the book's session, expression style: 25 gives 77, 212 gives 100") {
            val c = Connector()
            val f = celsiusFahrenheit(c)
            c.setValue(25L, User) shouldBe Either.Right(Unit)
            f.value() shouldBe 77L
            c.forgetValue(User)
            f.setValue(212L, User) shouldBe Either.Right(Unit)
            c.value() shouldBe 100L
        }

        test("the combinators compose and infer both ways") {
            val x = cConst(10L)
            val y = cConst(3L)
            cPlus(x, y).value() shouldBe 13L
            cMinus(x, y).value() shouldBe 7L
            cMul(x, y).value() shouldBe 30L
            val product = cMul(x, y)
            cDiv(product, y).value() shouldBe 10L
        }

        test("cDiv is Long division: 9/5 truncates to 1, as documented") {
            cDiv(cConst(9L), cConst(5L)).value() shouldBe 1L
        }
    })
