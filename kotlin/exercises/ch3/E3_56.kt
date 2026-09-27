// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.56

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * Exercise 3.56: the book's merge of two ordered streams into one
 * ordered result stream, eliminating repetitions: the smaller head goes
 * first, equal heads are kept once, and an empty argument lets the
 * other stream through.
 */
public fun merge(
    s1: LStream<Long>,
    s2: LStream<Long>,
): LStream<Long> = throw PendingSolution()

/**
 * The book's S: the positive integers with no prime factors other than
 * 2, 3, or 5, in ascending order without repetitions. It begins with 1,
 * and the rest is the merge of its doubles, its triples, and its
 * quintuples; the self-referential top level value shares one memoized
 * chain across every caller.
 */
public val hamming: LStream<Long> = throw PendingSolution()
