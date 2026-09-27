// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 93a

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_93aTest :
    FunSpec({
        test("the reduction runs once, on first inspection, and memoizes") {
            ex_2_93a() shouldBe Triple(0, 1, true)
        }

        test("an unreduced LazyRatF prints through its reduced pair") {
            val table = NumTable()
            installGenericArithmetic(table)
            installNeg(table)
            installPolynomialPackage(table)
            installPolyIsZero(table)
            installRationalFunctionPackage(table)
            val p1 = makePolynomial("x", listOf(Term(2, ZLong(1)), Term(0, ZLong(1))))
            val p2 = makePolynomial("x", listOf(Term(3, ZLong(1)), Term(0, ZLong(1))))
            val lazyRf =
                LazyRatF(p2, p1) { n, d ->
                    arrow.core.raise
                        .either { reduce(table, n, d) }
                        .map { r -> r.num to r.den }
                        .getOrNull() ?: (n to d)
                }
            lazyRf.toString() shouldBe "(1*x^3 + 1 in x)/(1*x^2 + 1 in x)"
        }
    })
