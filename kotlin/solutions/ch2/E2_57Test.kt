// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.57

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_57Test :
    FunSpec({
        test("accessors separate a two-term sum and a longer tail") {
            val twoTerms = NaryExpr.Sum(listOf(NaryExpr.Var("x"), NaryExpr.Num(3)))
            addend(twoTerms) shouldBe NaryExpr.Var("x")
            augend(twoTerms) shouldBe NaryExpr.Num(3)

            val threeTerms = NaryExpr.Sum(listOf(NaryExpr.Var("x"), NaryExpr.Var("y"), NaryExpr.Num(3)))
            augend(threeTerms) shouldBe NaryExpr.Sum(listOf(NaryExpr.Var("y"), NaryExpr.Num(3)))
        }
        test("the three-factor product rule constructs the expected derivative tree") {
            val expression =
                NaryExpr.Product(
                    listOf(
                        NaryExpr.Var("x"),
                        NaryExpr.Var("y"),
                        NaryExpr.Sum(listOf(NaryExpr.Var("x"), NaryExpr.Num(3))),
                    ),
                )
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
            derivN(expression, "x") shouldBe expected
            ex_2_57() shouldBe expected
        }
        test("the two-term product rule retains the base case") {
            derivN(NaryExpr.Product(listOf(NaryExpr.Var("x"), NaryExpr.Var("y"))), "x") shouldBe NaryExpr.Var("y")
        }
    })
