// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.3.2

package sicp.ch2.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/**
 * The section's base algebraic-expression type: a number, a variable, a
 * two-term sum, or a two-term product. `sum?`/`product?`/`variable?`
 * collapse into the sealed hierarchy's own `when` branches; `addend`,
 * `augend`, `multiplier`, and `multiplicand` become [Sum.a1]/[Sum.a2] and
 * [Product.a1]/[Product.a2].
 */
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

/** Prints an [Expr] in the book's parenthesized prefix notation. */
public fun printExpr(e: Expr): String =
    when (e) {
        is Expr.Num -> e.n.toString()
        is Expr.Var -> e.name
        is Expr.Sum -> "(+ ${printExpr(e.a1)} ${printExpr(e.a2)})"
        is Expr.Product -> "(* ${printExpr(e.a1)} ${printExpr(e.a2)})"
    }

/** The naive version: builds sums and products without any simplification. */
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
    e: Expr,
    n: Long,
): Boolean = e is Expr.Num && e.n == n

/** `make-sum`: `0` is the additive identity, and two literals fold into one. */
public fun makeSum(
    a1: Expr,
    a2: Expr,
): Expr =
    when {
        isNumber(a1, 0) -> a2
        isNumber(a2, 0) -> a1
        a1 is Expr.Num && a2 is Expr.Num -> Expr.Num(a1.n + a2.n)
        else -> Expr.Sum(a1, a2)
    }

/** `make-product`: `0` annihilates, `1` is the multiplicative identity, and two literals fold into one. */
public fun makeProduct(
    m1: Expr,
    m2: Expr,
): Expr =
    when {
        isNumber(m1, 0) || isNumber(m2, 0) -> Expr.Num(0)
        isNumber(m1, 1) -> m2
        isNumber(m2, 1) -> m1
        m1 is Expr.Num && m2 is Expr.Num -> Expr.Num(m1.n * m2.n)
        else -> Expr.Product(m1, m2)
    }

/** `deriv`, unchanged in shape from [derivUnsimplified]; only the constructors it calls differ. */
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

        test("derivUnsimplified of (+ x 3) w.r.t. x is (+ 1 0)") {
            printExpr(derivUnsimplified(sum, "x")) shouldBe "(+ 1 0)"
        }
        test("derivUnsimplified of (* x y) w.r.t. x is (+ (* x 0) (* 1 y))") {
            printExpr(derivUnsimplified(product, "x")) shouldBe "(+ (* x 0) (* 1 y))"
        }
        test("derivUnsimplified of (* (* x y) (+ x 3)) w.r.t. x nests both rules") {
            printExpr(derivUnsimplified(nested, "x")) shouldBe "(+ (* (* x y) (+ 1 0)) (* (+ (* x 0) (* 1 y)) (+ x 3)))"
        }
        test("deriv (simplified) of (+ x 3) w.r.t. x is 1") {
            printExpr(deriv(sum, "x")) shouldBe "1"
        }
        test("deriv (simplified) of (* x y) w.r.t. x is y") {
            printExpr(deriv(product, "x")) shouldBe "y"
        }
        test("deriv (simplified) of (* (* x y) (+ x 3)) w.r.t. x is (+ (* x y) (* y (+ x 3)))") {
            printExpr(deriv(nested, "x")) shouldBe "(+ (* x y) (* y (+ x 3)))"
        }
    })
