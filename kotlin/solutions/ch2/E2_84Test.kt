// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 84

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_84Test :
    FunSpec({
        test("successive raising lifts the lower argument to the higher package") {
            ex_2_84() shouldBe true
        }

        test("an operation with no common-level handler still raises NoMethod") {
            val table = NumTable()
            installGenericArithmetic(table)
            installRaise(table)
            val result =
                arrow.core.raise.either {
                    applyGenericRaising(table, "exp", listOf(ZLong(2), Complex(Rect(1.0, 0.0))))
                }
            result.leftOrNull() shouldBe GenError.NoMethod("exp", listOf("complex", "complex"))
        }
    })
