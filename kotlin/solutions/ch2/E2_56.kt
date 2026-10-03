// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.56

package sicp.ch2.exercises

/** Section 2.3.2's `Expr`, extended with `Pow` for this exercise only. */
public sealed interface PowExpr {
    public data class Num(
        val n: Long,
    ) : PowExpr

    public data class Var(
        val name: String,
    ) : PowExpr

    public data class Sum(
        val a1: PowExpr,
        val a2: PowExpr,
    ) : PowExpr

    public data class Product(
        val a1: PowExpr,
        val a2: PowExpr,
    ) : PowExpr

    public data class Pow(
        val base: PowExpr,
        val n: Long,
    ) : PowExpr
}

private fun isNumber(
    e: PowExpr,
    n: Long,
): Boolean = e is PowExpr.Num && e.n == n

public fun makeSumPow(
    a1: PowExpr,
    a2: PowExpr,
): PowExpr =
    when {
        isNumber(a1, 0) -> a2
        isNumber(a2, 0) -> a1
        a1 is PowExpr.Num && a2 is PowExpr.Num -> PowExpr.Num(a1.n + a2.n)
        else -> PowExpr.Sum(a1, a2)
    }

public fun makeProductPow(
    m1: PowExpr,
    m2: PowExpr,
): PowExpr =
    when {
        isNumber(m1, 0) || isNumber(m2, 0) -> PowExpr.Num(0)
        isNumber(m1, 1) -> m2
        isNumber(m2, 1) -> m1
        m1 is PowExpr.Num && m2 is PowExpr.Num -> PowExpr.Num(m1.n * m2.n)
        else -> PowExpr.Product(m1, m2)
    }

/** The book's rules: anything to the 0th power is 1, anything to the 1st power is itself. */
public fun makeExponentiation(
    base: PowExpr,
    n: Long,
): PowExpr =
    when (n) {
        0L -> PowExpr.Num(1)
        1L -> base
        else -> PowExpr.Pow(base, n)
    }

public fun derivPow(
    exp: PowExpr,
    variable: String,
): PowExpr =
    when (exp) {
        is PowExpr.Num -> {
            PowExpr.Num(0)
        }

        is PowExpr.Var -> {
            PowExpr.Num(if (exp.name == variable) 1 else 0)
        }

        is PowExpr.Sum -> {
            makeSumPow(derivPow(exp.a1, variable), derivPow(exp.a2, variable))
        }

        is PowExpr.Product -> {
            makeSumPow(
                makeProductPow(exp.a1, derivPow(exp.a2, variable)),
                makeProductPow(derivPow(exp.a1, variable), exp.a2),
            )
        }

        is PowExpr.Pow -> {
            makeProductPow(
                makeProductPow(PowExpr.Num(exp.n), makeExponentiation(exp.base, exp.n - 1)),
                derivPow(exp.base, variable),
            )
        }
    }

/** The derivative tree for the power-rule sample. */
public fun ex_2_56(): PowExpr = derivPow(PowExpr.Pow(PowExpr.Var("x"), 3L), "x")
