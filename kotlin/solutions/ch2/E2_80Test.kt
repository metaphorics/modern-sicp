// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 80

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_80Test :
    FunSpec({
        test("=zero? answers for every level of the tower") {
            ex_2_80().all { it } shouldBe true
            ex_2_80().size shouldBe 8
        }

        test("a complex with a nonzero imaginary part is not zero") {
            val table = NumTable()
            installIsZero(table)
            val result = arrow.core.raise.either { isZeroQ(table, Complex(Polar(0.0, 1.0))) }
            result.getOrNull() shouldBe true
        }
    })
