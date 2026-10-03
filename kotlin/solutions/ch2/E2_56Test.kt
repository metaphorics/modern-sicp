// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.56

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_56Test :
    FunSpec({
        test("makeExponentiation folds powers zero and one, retaining other exponents") {
            makeExponentiation(PowExpr.Var("x"), 0L) shouldBe PowExpr.Num(1)
            makeExponentiation(PowExpr.Var("x"), 1L) shouldBe PowExpr.Var("x")
            makeExponentiation(PowExpr.Var("x"), 2L) shouldBe PowExpr.Pow(PowExpr.Var("x"), 2L)
        }
        test("the power rule for the cube produces the expected expression tree") {
            derivPow(PowExpr.Pow(PowExpr.Var("x"), 3L), "x") shouldBe
                PowExpr.Product(PowExpr.Num(3L), PowExpr.Pow(PowExpr.Var("x"), 2L))
        }
        test("the power rule for exponent one simplifies to one") {
            derivPow(PowExpr.Pow(PowExpr.Var("x"), 1L), "x") shouldBe PowExpr.Num(1)
        }
        test("derivPow still handles plain sums and products") {
            val sum = PowExpr.Sum(PowExpr.Var("x"), PowExpr.Num(3))
            derivPow(sum, "x") shouldBe PowExpr.Num(1)
        }
        test("ex_2_56 matches the hand-computed derivative tree") {
            ex_2_56() shouldBe PowExpr.Product(PowExpr.Num(3L), PowExpr.Pow(PowExpr.Var("x"), 2L))
        }
    })
