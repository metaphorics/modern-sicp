// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.36

package sicp.ch2.exercises

/**
 * The book's `accumulate-n` with the missing expressions filled in: take
 * the first element of every sequence, accumulate them, and recurse on
 * the rests until every sequence is exhausted.
 */
public fun accumulateN(
    op: (Long, Long) -> Long,
    init: Long,
    seqs: List<List<Long>>,
): List<Long> =
    if (seqs.any { it.isEmpty() }) {
        emptyList()
    } else {
        listOf(accumulateList(op, init, seqs.map { it.first() })) + accumulateN(op, init, seqs.map { it.drop(1) })
    }

/** `accumulateN(+, 0, ...)` over the book's four sequences returns `(22 26 30)`. */
public fun ex_2_36(): List<Long> =
    accumulateN(
        { x, acc -> x + acc },
        0L,
        listOf(listOf(1L, 2L, 3L), listOf(4L, 5L, 6L), listOf(7L, 8L, 9L), listOf(10L, 11L, 12L)),
    )
