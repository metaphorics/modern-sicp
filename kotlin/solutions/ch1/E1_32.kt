// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.32

package sicp.ch1.exercises

public fun accumulate(
    combiner: (Long, Long) -> Long,
    nullValue: Long,
    term: (Long) -> Long,
    a: Long,
    next: (Long) -> Long,
    b: Long,
): Long = if (a > b) nullValue else combiner(term(a), accumulate(combiner, nullValue, term, next(a), next, b))

/** Part (b): the iterative shape, the same choice as exercises 1.30 and 1.31. */
public fun accumulateIterative(
    combiner: (Long, Long) -> Long,
    nullValue: Long,
    term: (Long) -> Long,
    a: Long,
    next: (Long) -> Long,
    b: Long,
): Long {
    tailrec fun iter(
        a: Long,
        result: Long,
    ): Long = if (a > b) result else iter(next(a), combiner(result, term(a)))
    return iter(a, nullValue)
}

private fun cube(x: Long): Long = x * x * x

private fun identity(x: Long): Long = x

private fun inc(n: Long): Long = n + 1L

public fun sumViaAccumulate(
    a: Long,
    b: Long,
): Long = accumulate({ x, y -> x + y }, 0L, ::cube, a, ::inc, b)

public fun factorialViaAccumulate(n: Long): Long = accumulate({ x, y -> x * y }, 1L, ::identity, 1L, ::inc, n)

public fun ex_1_32(): Pair<Long, Long> = Pair(sumViaAccumulate(1L, 10L), factorialViaAccumulate(6L))
