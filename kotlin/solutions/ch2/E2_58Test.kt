// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.58

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_58Test :
    FunSpec({
        test("part a) parses the book's fully-parenthesized example") {
            val expected =
                Expr.Sum(
                    Expr.Var("x"),
                    Expr.Product(Expr.Num(3), Expr.Sum(Expr.Var("x"), Expr.Sum(Expr.Var("y"), Expr.Num(2)))),
                )
            parseFullyParenthesized("(x + (3 * (x + (y + 2))))") shouldBe expected
        }
        test("part b) applies standard precedence: * binds tighter than +, left-associatively") {
            val expected =
                Expr.Sum(
                    Expr.Var("x"),
                    Expr.Product(Expr.Num(3), Expr.Sum(Expr.Sum(Expr.Var("x"), Expr.Var("y")), Expr.Num(2))),
                )
            parseWithPrecedence("x + 3 * (x + y + 2)") shouldBe expected
        }
        test("both parsers agree on the differentiated result, despite differently associated sums") {
            val a = parseFullyParenthesized("(x + (3 * (x + (y + 2))))")
            val b = parseWithPrecedence("x + 3 * (x + y + 2)")
            printExpr(deriv(a, "x")) shouldBe printExpr(deriv(b, "x"))
        }
        test("deriv differentiates the precedence-parsed result without any change") {
            printExpr(deriv(parseWithPrecedence("x + 3 * (x + y + 2)"), "x")) shouldBe "4"
        }
        test("ex_2_58 matches the hand-computed derivative") {
            ex_2_58() shouldBe "4"
        }
    })
