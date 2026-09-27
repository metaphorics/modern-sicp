// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 89

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_89Test :
    FunSpec({
        test("dense term lists match the sparse results for sum and product") {
            ex_2_89() shouldBe true
        }

        test("dense adjoin keeps explicit zero coefficients so orders stay aligned") {
            val l = DenseTerms.adjoin(Term(2, ZLong(1)), DenseTerms.empty())
            val l2 = DenseTerms.adjoin(Term(1, ZLong(0)), l)
            DenseTerms.first(l2) shouldBe Term(1, ZLong(0))
            DenseTerms.rest(l2) shouldBe l
        }
    })
