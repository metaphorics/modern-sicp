// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 97

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_97Test :
    FunSpec({
        test("rf1 + rf2 comes out correctly reduced, the book's displayed answer") {
            ex_2_97() shouldBe "(1*x^3 + 2*x^2 + 3*x + 1 in x)/(1*x^4 + 1*x^3 + (-1)*x + (-1) in x)"
        }

        test("reduceIntegers mirrors the original makeRat") {
            reduceIntegers(48, 18) shouldBe (8L to 3L)
        }
    })
