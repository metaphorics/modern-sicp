// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 92

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_92Test :
    FunSpec({
        test("the product's cubic coefficient and the mixed add both land") {
            ex_2_92() shouldBe (true to true)
        }

        test("rebasing refuses to lower a dominant variable") {
            val table = NumTable()
            installGenericArithmetic(table)
            installPolynomialPackage(table)
            installNeg(table)
            installPolyIsZero(table)
            val inX = makePolynomial("x", listOf(Term(1, ZLong(1))))
            val result = arrow.core.raise.either { rebase92(inX, "y") }
            result.leftOrNull() shouldBe GenError.BadArgs("rebase", "cannot lower x into y without expanding")
        }
    })
