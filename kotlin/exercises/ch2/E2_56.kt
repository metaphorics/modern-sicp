// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.56

package sicp.ch2.exercises

/** An expression tree extended with integer powers for this exercise. */
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

/**
 * Exercise 2.56 (Class A): extend the typed differentiator with a `Pow`
 * expression node and the power rule. Preserve constructor simplifications
 * for exponents zero and one, then differentiate the cube of `x`.
 *
 * The scaffold returns the derivative expression tree for inspection.
 */
public fun ex_2_56(): PowExpr = throw PendingExercise()
