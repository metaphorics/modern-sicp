// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.73

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_73Test :
    FunSpec({
        test("the installed table differentiates sums, products, and powers") {
            ex_2_73() shouldBe
                Triple(
                    OperatorExpr.Num(1),
                    OperatorExpr.Var("y"),
                    OperatorExpr.Product(OperatorExpr.Num(3), OperatorExpr.Pow(OperatorExpr.Var("x"), 2L)),
                )
        }

        test("installing another rule leaves existing rule results unchanged") {
            val table = DerivTable()
            installSumRule(table)
            installProductRule(table)
            val sum = OperatorExpr.Sum(OperatorExpr.Var("x"), OperatorExpr.Num(3))
            val sumBefore = derivDataDirected(sum, "x", table)
            installPowRule(table)
            val sumAfter = derivDataDirected(sum, "x", table)
            sumBefore shouldBe sumAfter
        }

        test("a missing operator rule fails with a typed host error") {
            val table = DerivTable()
            val failure =
                runCatching {
                    derivDataDirected(OperatorExpr.Product(OperatorExpr.Var("x"), OperatorExpr.Num(2)), "x", table)
                }.exceptionOrNull()
            (failure is IllegalStateException) shouldBe true
        }
    })
