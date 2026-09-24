// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 94

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_94Test :
    FunSpec({
        test("the book's test pair comes out to x^2 - x up to sign") {
            ex_2_94() shouldBe "(-1)*x^2 + 1*x in x"
        }

        test("the integer gcd answers through the same generic operation") {
            val table = NumTable()
            installGreatestCommonDivisor(table)
            val result =
                arrow.core.raise.either { applyGeneric(table, "greatest-common-divisor", listOf(ZLong(12), ZLong(18))) }
            result.getOrNull() shouldBe ZLong(6)
        }
    })
