// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.57

package sicp.ch2.exercises

/** An expression tree whose sums and products hold any number of terms. */
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

/**
 * Exercise 2.57: extend differentiation to sums and products with two or more
 * terms. Preserve the derivative dispatcher by changing only how a sum's
 * remaining terms and a product's remaining factors are represented.
 *
 * The scaffold returns the derivative tree for a three-factor product.
 */
public fun ex_2_57(): NaryExpr = throw PendingExercise()
