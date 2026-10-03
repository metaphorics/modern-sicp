// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.58

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_58Test :
    FunSpec({
        test("part a parses the fully parenthesized infix example") {
            val expected =
                Expr.Sum(
                    Expr.Var("x"),
                    Expr.Product(Expr.Num(3), Expr.Sum(Expr.Var("x"), Expr.Sum(Expr.Var("y"), Expr.Num(2)))),
                )
            parseFullyParenthesized("(x + (3 * (x + (y + 2))))") shouldBe expected
        }
        test("part b applies multiplication precedence and left-associative addition") {
            val expected =
                Expr.Sum(
                    Expr.Var("x"),
                    Expr.Product(Expr.Num(3), Expr.Sum(Expr.Sum(Expr.Var("x"), Expr.Var("y")), Expr.Num(2))),
                )
            parseWithPrecedence("x + 3 * (x + y + 2)") shouldBe expected
        }
        test("both parsers produce structurally equal derivatives") {
            val parenthesized = parseFullyParenthesized("(x + (3 * (x + (y + 2))))")
            val precedenceAware = parseWithPrecedence("x + 3 * (x + y + 2)")
            deriv(parenthesized, "x") shouldBe deriv(precedenceAware, "x")
        }
        test("differentiation simplifies the precedence-parsed derivative to four") {
            deriv(parseWithPrecedence("x + 3 * (x + y + 2)"), "x") shouldBe Expr.Num(4)
        }
        test("ex_2_58 returns the hand-computed derivative tree") {
            ex_2_58() shouldBe Expr.Num(4)
        }
    })
