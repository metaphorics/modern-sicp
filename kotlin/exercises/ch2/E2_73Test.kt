// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.73

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_73Test :
    FunSpec({
        test("Exercise 2.73 returns the sum, product, and power derivative trees").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_73() shouldBe
                Triple(
                    OperatorExpr.Num(1L),
                    OperatorExpr.Var("y"),
                    OperatorExpr.Product(OperatorExpr.Num(3L), OperatorExpr.Pow(OperatorExpr.Var("x"), 2L)),
                )
        }
    })
