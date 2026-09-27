// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.57

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_57Test :
    FunSpec({
        test("addend/augend of a two-term sum are the two terms directly") {
            val sum = NaryExpr.Sum(listOf(NaryExpr.Var("x"), NaryExpr.Num(3)))
            addend(sum) shouldBe NaryExpr.Var("x")
            augend(sum) shouldBe NaryExpr.Num(3)
        }
        test("augend of a three-term sum is the sum of the rest") {
            val sum = NaryExpr.Sum(listOf(NaryExpr.Var("x"), NaryExpr.Var("y"), NaryExpr.Num(3)))
            augend(sum) shouldBe NaryExpr.Sum(listOf(NaryExpr.Var("y"), NaryExpr.Num(3)))
        }
        test("derivN of a three-factor product matches the book's worked example") {
            val expr =
                NaryExpr.Product(
                    listOf(NaryExpr.Var("x"), NaryExpr.Var("y"), NaryExpr.Sum(listOf(NaryExpr.Var("x"), NaryExpr.Num(3)))),
                )
            printNaryExpr(derivN(expr, "x")) shouldBe "(+ (* x y) (* y (+ x 3)))"
        }
        test("derivN of a two-term product still matches the base two-argument rule") {
            val expr = NaryExpr.Product(listOf(NaryExpr.Var("x"), NaryExpr.Var("y")))
            printNaryExpr(derivN(expr, "x")) shouldBe "y"
        }
        test("ex_2_57 matches the hand-computed derivative") {
            ex_2_57() shouldBe "(+ (* x y) (* y (+ x 3)))"
        }
    })
