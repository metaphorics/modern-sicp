// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.73

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_73Test :
    FunSpec({
        test("ex_2_73 differentiates a sum, a product, and a power, all through the table") {
            ex_2_73() shouldBe Triple("1", "y", "(* 3 (** x 2))")
        }

        test("adding a rule is one put; deriving the same expressions before and after installing the power rule agrees where both apply") {
            val table = DerivTable()
            installSumRule(table)
            installProductRule(table)
            val sumBefore = printOperatorExpr(derivDataDirected(OperatorExpr.Sum(OperatorExpr.Var("x"), OperatorExpr.Num(3)), "x", table))
            installPowRule(table)
            val sumAfter = printOperatorExpr(derivDataDirected(OperatorExpr.Sum(OperatorExpr.Var("x"), OperatorExpr.Num(3)), "x", table))
            sumBefore shouldBe sumAfter
        }

        test("an expression whose operator has no installed rule fails loudly, not silently") {
            val table = DerivTable()
            val failure = runCatching { derivDataDirected(OperatorExpr.Product(OperatorExpr.Var("x"), OperatorExpr.Num(2)), "x", table) }
            failure.isFailure shouldBe true
        }
    })
