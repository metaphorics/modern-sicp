// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.57

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_57Test :
    FunSpec({
        test("Exercise 2.57 returns the derivative tree for a three-factor product").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val expected =
                NaryExpr.Sum(
                    listOf(
                        NaryExpr.Product(listOf(NaryExpr.Var("x"), NaryExpr.Var("y"))),
                        NaryExpr.Product(
                            listOf(
                                NaryExpr.Var("y"),
                                NaryExpr.Sum(listOf(NaryExpr.Var("x"), NaryExpr.Num(3))),
                            ),
                        ),
                    ),
                )
            ex_2_57() shouldBe expected
        }
    })
