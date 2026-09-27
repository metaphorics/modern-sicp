// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.33

package sicp.ch2.exercises

/** The book's `accumulate`, hand-rolled over `List` exactly as 2.2.3 defines it. */
public fun <T, R> accumulateList(
    op: (T, R) -> R,
    initial: R,
    sequence: List<T>,
): R = if (sequence.isEmpty()) initial else op(sequence.first(), accumulateList(op, initial, sequence.drop(1)))

/** The book's `map` defined as an accumulation: cons each mapped head onto the mapped rest. */
public fun mapViaAccumulate(
    p: (Long) -> Long,
    sequence: List<Long>,
): List<Long> = accumulateList({ x, acc -> listOf(p(x)) + acc }, emptyList(), sequence)

/** The book's `filter` defined as an accumulation over the elements that pass. */
public fun filterViaAccumulate(
    predicate: (Long) -> Boolean,
    sequence: List<Long>,
): List<Long> = accumulateList({ x, acc -> if (predicate(x)) listOf(x) + acc else acc }, emptyList(), sequence)

/** The book's `append` defined as an accumulation over the first list. */
public fun appendViaAccumulate(
    seq1: List<Long>,
    seq2: List<Long>,
): List<Long> = accumulateList({ x, acc -> listOf(x) + acc }, seq2, seq1)

/** The book's `length` defined as an accumulation: count each element as one. */
public fun lengthViaAccumulate(sequence: List<Long>): Long = accumulateList({ _, acc -> 1L + acc }, 0L, sequence)

/** The book's `flatmap`: map, then accumulate the results with append. */
public fun <T, R> flatMapSeq(
    proc: (T) -> List<R>,
    sequence: List<T>,
): List<R> = accumulateList({ x: List<R>, acc: List<R> -> x + acc }, emptyList(), sequence.map(proc))

/** `mapViaAccumulate` squaring `(1 2 3 4)` to `(1 4 9 16)`. */
public fun ex_2_33(): List<Long> = mapViaAccumulate({ x -> x * x }, listOf(1L, 2L, 3L, 4L))
