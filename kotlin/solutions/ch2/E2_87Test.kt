// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 87

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_87Test :
    FunSpec({
        test("=zero? for polynomials sees through nested coefficients") {
            ex_2_87() shouldBe true
        }

        test("a flat nonzero polynomial is not zero") {
            val table = NumTable()
            installGenericArithmetic(table)
            installPolynomialPackage(table)
            installPolyIsZero(table)
            val result =
                arrow.core.raise.either {
                    isZeroG(table, makePolynomial("x", listOf(Term(1, ZLong(3)))))
                }
            result.getOrNull() shouldBe false
        }
    })
