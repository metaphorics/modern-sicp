// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.9

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_09Test :
    FunSpec({
        test("the book's two factorials agree: factorial(6) is 720") {
            factorial(6L) shouldBe 720L
            factorialIter(6L) shouldBe 720L
        }

        test("small cases: factorial(1) and factorialIter(1) are 1") {
            factorial(1L) shouldBe 1L
            factorialIter(1L) shouldBe 1L
        }

        test("factIter threads product and counter through parameters") {
            factIter(1L, 1L, 3L) shouldBe 6L
            factIter(5L, 2L, 1L) shouldBe 5L
        }
    })
