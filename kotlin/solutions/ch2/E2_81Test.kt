// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 81

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_81Test :
    FunSpec({
        test("the loop is observable, the plain dispatch correct, the fix honest") {
            ex_2_81() shouldBe Triple(true, true, true)
        }

        test("the fixed dispatch still coerces mixed levels") {
            val table = NumTable()
            installGenericArithmetic(table)
            val coercions = CoercionTable()
            coercions.putCoercion(ZLong::class, Complex::class) { z -> Complex(Rect((z as ZLong).n.toDouble(), 0.0)) }
            val result =
                arrow.core.raise.either {
                    applyGenericNoSelfCoercion(table, coercions, "add", listOf(ZLong(2), Complex(Rect(3.0, 0.0))))
                }
            result.getOrNull() shouldBe Complex(Rect(5.0, 0.0))
        }
    })
