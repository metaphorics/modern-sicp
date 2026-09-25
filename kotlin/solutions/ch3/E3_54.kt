// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.54

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream

/**
 * Exercise 3.54: define mul-streams, analogous to add-streams, producing
 * the elementwise product of its two input streams; either stream running
 * out ends the result. Elementwise products multiply heads and recurse
 * on the tails, one delayed cons per node.
 */
public fun mulStreams(
    s1: LStream<Long>,
    s2: LStream<Long>,
): LStream<Long> =
    when {
        s1 is LStream.Empty || s2 is LStream.Empty -> {
            LStream.Empty
        }

        s1 is LStream.Cons && s2 is LStream.Cons -> {
            consStream(Math.multiplyExact(s1.head, s2.head)) { mulStreams(s1.tail, s2.tail) }
        }

        else -> {
            LStream.Empty
        }
    }

/**
 * The book's factorials: the stream whose n-th element, counting from 0,
 * is (n + 1) factorial. It is factorials itself multiplied elementwise by
 * the integers, shifted one place: each new element multiplies the
 * previous factorial by the next integer, and the self-referential top
 * level value shares one memoized chain across every caller.
 */
public val factorials: LStream<Long> =
    consStream(1L) { zipStream(factorials, integers) { f, n -> Math.multiplyExact(f, n) } }
