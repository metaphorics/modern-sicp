// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.70

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * Exercise 3.70: define [mergeWeighted], which is like the 3.56 merge
 * except that it takes an additional [weight] procedure computing the
 * weight of a pair and orders the merged stream by it. Using this,
 * generalize [pairs] to [weightedPairs], which takes two streams and a
 * weighting function and generates the pairs ordered according to
 * weight. Use it to generate
 * 1. the stream of all pairs of positive integers `(i, j)` with
 *    `i <= j`, ordered according to the sum `i + j`, and
 * 2. the stream of all pairs of positive integers `(i, j)` with
 *    `i <= j`, where neither `i` nor `j` is divisible by 2, 3, or 5,
 *    ordered according to `2i + 3j + 5ij` (the components come from
 *    [integersNo235]).
 */
public fun mergeWeighted(
    s1: LStream<Pair<Long, Long>>,
    s2: LStream<Pair<Long, Long>>,
    weight: (Pair<Long, Long>) -> Long,
): LStream<Pair<Long, Long>> = throw PendingSolution()

public fun weightedPairs(
    s: LStream<Long>,
    t: LStream<Long>,
    weight: (Pair<Long, Long>) -> Long,
): LStream<Pair<Long, Long>> = throw PendingSolution()

/** 3.70(b)'s component stream: the positive integers that 2, 3, and 5
 * fail to divide. */
public val integersNo235: LStream<Long> = throw PendingSolution()
