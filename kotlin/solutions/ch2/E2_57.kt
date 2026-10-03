// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.57

package sicp.ch2.exercises

/** Section 2.3.2's `Expr`, with `Sum`/`Product` widened to two or more terms. */
public sealed interface NaryExpr {
    public data class Num(
        val n: Long,
    ) : NaryExpr

    public data class Var(
        val name: String,
    ) : NaryExpr

    public data class Sum(
        val terms: List<NaryExpr>,
    ) : NaryExpr

    public data class Product(
        val factors: List<NaryExpr>,
    ) : NaryExpr
}

private fun isNumber(
    e: NaryExpr,
    n: Long,
): Boolean = e is NaryExpr.Num && e.n == n

/** The first term of a sum. */
public fun addend(s: NaryExpr.Sum): NaryExpr = s.terms.first()

/** The rest of the terms of a sum: a single remaining term stands for itself. */
public fun augend(s: NaryExpr.Sum): NaryExpr = if (s.terms.size == 2) s.terms[1] else NaryExpr.Sum(s.terms.drop(1))

/** The first factor of a product. */
public fun multiplier(p: NaryExpr.Product): NaryExpr = p.factors.first()

/** The rest of the factors of a product: a single remaining factor stands for itself. */
public fun multiplicand(p: NaryExpr.Product): NaryExpr = if (p.factors.size == 2) p.factors[1] else NaryExpr.Product(p.factors.drop(1))

public fun makeSum2(
    a1: NaryExpr,
    a2: NaryExpr,
): NaryExpr =
    when {
        isNumber(a1, 0) -> a2
        isNumber(a2, 0) -> a1
        a1 is NaryExpr.Num && a2 is NaryExpr.Num -> NaryExpr.Num(a1.n + a2.n)
        else -> NaryExpr.Sum(listOf(a1, a2))
    }

public fun makeProduct2(
    m1: NaryExpr,
    m2: NaryExpr,
): NaryExpr =
    when {
        isNumber(m1, 0) || isNumber(m2, 0) -> NaryExpr.Num(0)
        isNumber(m1, 1) -> m2
        isNumber(m2, 1) -> m1
        m1 is NaryExpr.Num && m2 is NaryExpr.Num -> NaryExpr.Num(m1.n * m2.n)
        else -> NaryExpr.Product(listOf(m1, m2))
    }

/** `derivN`'s shape is exactly [derivPow]'s two-argument shape, unchanged; only [addend]/[augend]/[multiplier]/[multiplicand] widen. */
public fun derivN(
    exp: NaryExpr,
    variable: String,
): NaryExpr =
    when (exp) {
        is NaryExpr.Num -> {
            NaryExpr.Num(0)
        }

        is NaryExpr.Var -> {
            NaryExpr.Num(if (exp.name == variable) 1 else 0)
        }

        is NaryExpr.Sum -> {
            makeSum2(derivN(addend(exp), variable), derivN(augend(exp), variable))
        }

        is NaryExpr.Product -> {
            makeSum2(
                makeProduct2(multiplier(exp), derivN(multiplicand(exp), variable)),
                makeProduct2(derivN(multiplier(exp), variable), multiplicand(exp)),
            )
        }
    }

/** The derivative tree for the section's three-factor product example. */
public fun ex_2_57(): NaryExpr {
    val expression =
        NaryExpr.Product(
            listOf(
                NaryExpr.Var("x"),
                NaryExpr.Var("y"),
                NaryExpr.Sum(listOf(NaryExpr.Var("x"), NaryExpr.Num(3))),
            ),
        )
    return derivN(expression, "x")
}
