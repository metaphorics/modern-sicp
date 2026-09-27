// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.21

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_21Test :
    FunSpec({
        test("the smallest divisors of 199, 1999, 19999") {
            ex_1_21() shouldBe Triple(199L, 1999L, 7L)
        }
        test("199 and 1999 are their own smallest divisor, so they are prime") {
            smallestDivisor(199L) shouldBe 199L
            smallestDivisor(1999L) shouldBe 1999L
        }
    })
