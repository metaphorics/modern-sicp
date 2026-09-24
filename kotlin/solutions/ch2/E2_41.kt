// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.41

package sicp.ch2.exercises

/**
 * All ordered triples of distinct positive integers `i < j < k <= n`
 * whose sum is [s]: for each i and each j past it, keep the k values
 * that complete the sum.
 */
public fun tripleSum(
    n: Long,
    s: Long,
): List<List<Long>> =
    flatMapSeq(
        { i ->
            flatMapSeq(
                { j -> ((j + 1L)..n).filter { k -> i + j + k == s }.map { k -> listOf(i, j, k) } },
                ((i + 1L)..n).toList(),
            )
        },
        (1L..n).toList(),
    )

/** `tripleSum(5, 10)` finds `(1 4 5)` and `(2 3 5)`. */
public fun ex_2_41(): List<List<Long>> = tripleSum(5L, 10L)
