// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, shared algebraic-expression type for the section 2.3.2 exercises

package sicp.ch2.exercises

/**
 * The section's base algebraic-expression type: a number, a variable, a
 * two-term sum, or a two-term product. This is the running prose's `Expr`
 * (see `book/ch2/2.3.texi`), reused unchanged by exercise 2.58, whose
 * parser only builds new front ends for this same representation.
 * Exercises 2.56 and 2.57 each keep their own extended local copy instead
 * of editing this one, matching the section's one-representation-per-
 * exercise convention.
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

private fun isNumber(
    e: Expr,
    n: Long,
): Boolean = e is Expr.Num && e.n == n

/** The book's `make-sum`, simplifying the two identity and constant-folding cases. */
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

/** The book's `make-product`, simplifying the zero, identity, and constant-folding cases. */
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

/** The book's `deriv`, using the simplified constructors throughout. */
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
