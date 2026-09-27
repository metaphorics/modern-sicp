// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.56

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_56Test :
    FunSpec({
        test("makeExponentiation folds power 0 to 1 and power 1 to the base") {
            makeExponentiation(PowExpr.Var("x"), 0L) shouldBe PowExpr.Num(1)
            makeExponentiation(PowExpr.Var("x"), 1L) shouldBe PowExpr.Var("x")
            makeExponentiation(PowExpr.Var("x"), 2L) shouldBe PowExpr.Pow(PowExpr.Var("x"), 2L)
        }
        test("derivPow of x^3 w.r.t. x is 3 x^2") {
            printPowExpr(derivPow(PowExpr.Pow(PowExpr.Var("x"), 3L), "x")) shouldBe "(* 3 (** x 2))"
        }
        test("derivPow of x^1 w.r.t. x is 1, since makeExponentiation folds the exponent 0 away") {
            printPowExpr(derivPow(PowExpr.Pow(PowExpr.Var("x"), 1L), "x")) shouldBe "1"
        }
        test("derivPow still handles plain sums and products") {
            val sum = PowExpr.Sum(PowExpr.Var("x"), PowExpr.Num(3))
            printPowExpr(derivPow(sum, "x")) shouldBe "1"
        }
        test("ex_2_56 matches the hand-computed derivative of x^3") {
            ex_2_56() shouldBe "(* 3 (** x 2))"
        }
    })
