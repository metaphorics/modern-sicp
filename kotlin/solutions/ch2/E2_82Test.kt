// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 82

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_82Test :
    FunSpec({
        test("uniform coercion succeeds and the subset-only operation stays unreachable") {
            ex_2_82() shouldBe (true to true)
        }

        test("a direct hit needs no coercion at all") {
            val table = NumTable()
            installBlendOperations(table)
            val coercions = CoercionTable()
            installTowerCoercions(coercions)
            val result =
                arrow.core.raise.either {
                    applyGenericMultiCoerce(table, coercions, "blend", listOf(ZLong(1), Real(2.0), ZLong(3)))
                }
            result.getOrNull() shouldBe Real(6.0)
        }
    })
