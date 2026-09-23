// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.30

package sicp.ch1.exercises

public fun sumIterative(
    term: (Long) -> Long,
    a: Long,
    next: (Long) -> Long,
    b: Long,
): Long {
    tailrec fun iter(
        a: Long,
        result: Long,
    ): Long = if (a > b) result else iter(next(a), result + term(a))
    return iter(a, 0L)
}

private fun cube(x: Long): Long = x * x * x

private fun inc(n: Long): Long = n + 1L

public fun ex_1_30(): Long = sumIterative(::cube, 1L, ::inc, 10L)
