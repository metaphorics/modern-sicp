// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.54

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * Exercise 3.54: define mul-streams, analogous to add-streams, producing
 * the elementwise product of its two input streams; either stream running
 * out ends the result. Elementwise products multiply heads and recurse
 * on the tails, one delayed cons per node.
 */
public fun mulStreams(
    s1: LStream<Long>,
    s2: LStream<Long>,
): LStream<Long> = throw PendingSolution()

/**
 * The book's factorials: the stream whose n-th element, counting from 0,
 * is (n + 1) factorial. It is factorials itself multiplied elementwise by
 * the integers, shifted one place: each new element multiplies the
 * previous factorial by the next integer, and the self-referential top
 * level value shares one memoized chain across every caller.
 */
public val factorials: LStream<Long> = throw PendingSolution()
