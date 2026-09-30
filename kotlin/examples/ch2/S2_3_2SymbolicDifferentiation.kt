// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.3.2

package sicp.ch2.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/** A typed expression tree for numbers, names, sums, and products. */
public sealed interface Expr {
    public data class Num(
        val n: Long,
    ) : Expr

    public data class Var(
        val name: String,
    ) : Expr

    public data class Sum(
        val a1: Expr,
        val a2: Expr,
    ) : Expr

    public data class Product(
        val a1: Expr,
        val a2: Expr,
    ) : Expr
}

/** Differentiate by constructing an unsimplified result expression tree. */
public fun derivUnsimplified(
    exp: Expr,
    variable: String,
): Expr =
    when (exp) {
        is Expr.Num -> {
            Expr.Num(0)
        }

        is Expr.Var -> {
            Expr.Num(if (exp.name == variable) 1 else 0)
        }

        is Expr.Sum -> {
            Expr.Sum(derivUnsimplified(exp.a1, variable), derivUnsimplified(exp.a2, variable))
        }

        is Expr.Product -> {
            Expr.Sum(
                Expr.Product(exp.a1, derivUnsimplified(exp.a2, variable)),
                Expr.Product(derivUnsimplified(exp.a1, variable), exp.a2),
            )
        }
    }

private fun isNumber(
    expression: Expr,
    number: Long,
): Boolean = expression is Expr.Num && expression.n == number

/** Construct a sum, removing additive identities and folding constants. */
public fun makeSum(
    a1: Expr,
    a2: Expr,
): Expr =
    when {
        isNumber(a1, 0L) -> a2
        isNumber(a2, 0L) -> a1
        a1 is Expr.Num && a2 is Expr.Num -> Expr.Num(a1.n + a2.n)
        else -> Expr.Sum(a1, a2)
    }

/** Construct a product, applying zero, identity, and constant-folding rules. */
public fun makeProduct(
    m1: Expr,
    m2: Expr,
): Expr =
    when {
        isNumber(m1, 0L) || isNumber(m2, 0L) -> Expr.Num(0)
        isNumber(m1, 1L) -> m2
        isNumber(m2, 1L) -> m1
        m1 is Expr.Num && m2 is Expr.Num -> Expr.Num(m1.n * m2.n)
        else -> Expr.Product(m1, m2)
    }

/** Differentiate while simplifying each constructed sum and product. */
public fun deriv(
    exp: Expr,
    variable: String,
): Expr =
    when (exp) {
        is Expr.Num -> {
            Expr.Num(0)
        }

        is Expr.Var -> {
            Expr.Num(if (exp.name == variable) 1 else 0)
        }

        is Expr.Sum -> {
            makeSum(deriv(exp.a1, variable), deriv(exp.a2, variable))
        }

        is Expr.Product -> {
            makeSum(
                makeProduct(exp.a1, deriv(exp.a2, variable)),
                makeProduct(deriv(exp.a1, variable), exp.a2),
            )
        }
    }

public class S2_3_2SymbolicDifferentiationTest :
    FunSpec({
        val sum = Expr.Sum(Expr.Var("x"), Expr.Num(3))
        val product = Expr.Product(Expr.Var("x"), Expr.Var("y"))
        val nested = Expr.Product(Expr.Product(Expr.Var("x"), Expr.Var("y")), Expr.Sum(Expr.Var("x"), Expr.Num(3)))

        test("the unsimplified sum rule preserves both zero terms") {
            derivUnsimplified(sum, "x") shouldBe Expr.Sum(Expr.Num(1), Expr.Num(0))
        }
        test("the unsimplified product rule preserves both product terms") {
            derivUnsimplified(product, "x") shouldBe
                Expr.Sum(
                    Expr.Product(Expr.Var("x"), Expr.Num(0)),
                    Expr.Product(Expr.Num(1), Expr.Var("y")),
                )
        }
        test("nested unsimplified differentiation preserves both rule expansions") {
            derivUnsimplified(nested, "x") shouldBe
                Expr.Sum(
                    Expr.Product(
                        Expr.Product(Expr.Var("x"), Expr.Var("y")),
                        Expr.Sum(Expr.Num(1), Expr.Num(0)),
                    ),
                    Expr.Product(
                        Expr.Sum(
                            Expr.Product(Expr.Var("x"), Expr.Num(0)),
                            Expr.Product(Expr.Num(1), Expr.Var("y")),
                        ),
                        Expr.Sum(Expr.Var("x"), Expr.Num(3)),
                    ),
                )
        }
        test("simplification removes zero terms in a sum derivative") {
            deriv(sum, "x") shouldBe Expr.Num(1)
        }
        test("simplification removes the zero product term") {
            deriv(product, "x") shouldBe Expr.Var("y")
        }
        test("nested simplified differentiation preserves the product rule result") {
            deriv(nested, "x") shouldBe
                Expr.Sum(
                    Expr.Product(Expr.Var("x"), Expr.Var("y")),
                    Expr.Product(Expr.Var("y"), Expr.Sum(Expr.Var("x"), Expr.Num(3))),
                )
        }
    })
