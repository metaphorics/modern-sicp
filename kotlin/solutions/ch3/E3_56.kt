// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.56

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream

/**
 * Exercise 3.56: the book's merge of two ordered streams into one
 * ordered result stream, eliminating repetitions: the smaller head goes
 * first, equal heads are kept once, and an empty argument lets the
 * other stream through.
 */
public fun merge(
    s1: LStream<Long>,
    s2: LStream<Long>,
): LStream<Long> =
    when {
        s1 is LStream.Empty -> {
            s2
        }

        s2 is LStream.Empty -> {
            s1
        }

        s1 is LStream.Cons && s2 is LStream.Cons -> {
            when {
                s1.head < s2.head -> consStream(s1.head) { merge(s1.tail, s2) }
                s1.head > s2.head -> consStream(s2.head) { merge(s1, s2.tail) }
                else -> consStream(s1.head) { merge(s1.tail, s2.tail) }
            }
        }

        else -> {
            LStream.Empty
        }
    }

/**
 * The book's S: the positive integers with no prime factors other than
 * 2, 3, or 5, in ascending order without repetitions. It begins with 1,
 * and the rest is the merge of its doubles, its triples, and its
 * quintuples; the self-referential top level value shares one memoized
 * chain across every caller.
 */
public val hamming: LStream<Long> =
    consStream(1L) {
        merge(
            merge(scaleStream(hamming, 2L), scaleStream(hamming, 3L)),
            scaleStream(hamming, 5L),
        )
    }
