// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.31

package sicp.ch1.exercises

public fun product(
    term: (Long) -> Long,
    a: Long,
    next: (Long) -> Long,
    b: Long,
): Long = if (a > b) 1L else term(a) * product(term, next(a), next, b)

/** Part (b): the same recursive/iterative choice as exercise 1.30, this time for product. */
public fun productIterative(
    term: (Long) -> Long,
    a: Long,
    next: (Long) -> Long,
    b: Long,
): Long {
    tailrec fun iter(
        a: Long,
        result: Long,
    ): Long = if (a > b) result else iter(next(a), result * term(a))
    return iter(a, 1L)
}

private fun identity(x: Long): Long = x

private fun inc(n: Long): Long = n + 1L

public fun factorial(n: Long): Long = product(::identity, 1L, ::inc, n)

private fun productD(
    term: (Double) -> Double,
    a: Double,
    next: (Double) -> Double,
    b: Double,
): Double = if (a > b) 1.0 else term(a) * productD(term, next(a), next, b)

/** The `k`-th Wallis factor: `(k+2)/(k+1)` for even `k`, `(k+1)/(k+2)` for odd `k`, `k` from 1. */
private fun wallisFactor(k: Long): Double = if (k % 2L == 0L) (k + 2.0) / (k + 1.0) else (k + 1.0) / (k + 2.0)

public fun wallisPi(terms: Long): Double = 4.0 * productD({ i -> wallisFactor(i.toLong()) }, 1.0, { i -> i + 1.0 }, terms.toDouble())

public fun ex_1_31(): Pair<Long, Double> = Pair(factorial(6L), wallisPi(1000L))
