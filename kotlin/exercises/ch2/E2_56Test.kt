// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.56

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_56Test :
    FunSpec({
        test("Exercise 2.56 returns the power-rule derivative tree for the cube").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_56() shouldBe PowExpr.Product(PowExpr.Num(3L), PowExpr.Pow(PowExpr.Var("x"), 2L))
        }
    })
