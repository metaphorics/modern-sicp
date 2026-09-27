// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 79

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_79Test :
    FunSpec({
        test("equ? answers at every level and across levels") {
            ex_2_79() shouldBe true
        }

        test("rational equality compares cross-multiplied values") {
            val table = NumTable()
            installEquQueries(table)
            val result =
                arrow.core.raise.either {
                    equvAcross(table, qr(2, 4), qr(1, 2)) && !equvAcross(table, qr(2, 4), qr(1, 3))
                }
            result.getOrNull() shouldBe true
        }
    })
