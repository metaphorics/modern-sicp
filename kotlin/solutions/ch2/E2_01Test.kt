// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.1

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_01Test :
    FunSpec({
        test("both arguments negative normalizes to both positive") {
            makeRat(-3L, -9L) shouldBe Rational(1L, 3L)
        }
        test("a negative denominator moves its sign onto the numerator") {
            makeRat(1L, -2L) shouldBe Rational(-1L, 2L)
        }
        test("a negative numerator with a positive denominator is already normalized") {
            makeRat(-1L, 2L) shouldBe Rational(-1L, 2L)
        }
        test("ex_2_01 matches makeRat(-3, -9)") {
            ex_2_01() shouldBe Rational(1L, 3L)
        }
    })
