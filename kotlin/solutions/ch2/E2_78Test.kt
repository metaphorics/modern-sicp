// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 78

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_78Test :
    FunSpec({
        test("bare numbers ride the tower unwrapped and stay bare") {
            ex_2_78() shouldBe true
        }

        test("the revised tag reader reads host numbers and tower values") {
            typeTagOfAny(3L) shouldBe "integer"
            typeTagOfAny(2.5) shouldBe "real"
            typeTagOfAny(ZLong(1)) shouldBe "integer"
            typeTagOfAny(qr(1, 2)) shouldBe "rational"
            typeTagOfAny("x") shouldBe null
        }
    })
