// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 95

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_95Test :
    FunSpec({
        test("the trace shows the stalled division and no P1-shaped answer") {
            val report = ex_2_95()
            report[0] shouldBe "stalled=true"
            report[1] shouldBe "not-p1=true"
            report[2] shouldBe "gcd=none"
        }

        test("Q1 and Q2 come out as the book defines them") {
            val table = NumTable()
            installGenericArithmetic(table)
            installNeg(table)
            installPolynomialPackage(table)
            installPolyIsZero(table)
            arrow.core.raise
                .either { show(makeQ1(table)) }
                .getOrNull() shouldBe
                "11*x^4 + (-22)*x^3 + 18*x^2 + (-14)*x + 7 in x"
            arrow.core.raise
                .either { show(makeQ2(table)) }
                .getOrNull() shouldBe
                "13*x^3 + (-21)*x^2 + 3*x + 5 in x"
        }
    })
