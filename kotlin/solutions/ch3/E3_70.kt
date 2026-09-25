// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.70

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.streamHead
import sicp.runtime.streamTail

/**
 * Exercise 3.70: [mergeWeighted] is the 3.56 merge with a weight: the
 * smaller weight goes first. Equal weights send both elements out,
 * adjacent, so the multi-representation scans of 3.71 and 3.72 see every
 * duplicate. [weightedPairs] generalizes [pairs] the same way: the
 * diagonal pair, then [mergeWeighted] of the first row and the
 * [weightedPairs] of the tails. The weight must increase along each row
 * and down each column, so the merge always sees sorted inputs.
 */
public fun mergeWeighted(
    s1: LStream<Pair<Long, Long>>,
    s2: LStream<Pair<Long, Long>>,
    weight: (Pair<Long, Long>) -> Long,
): LStream<Pair<Long, Long>> {
    val a = s1.streamHead()
    val b = s2.streamHead()
    return when {
        a == null -> {
            s2
        }

        b == null -> {
            s1
        }

        else -> {
            val w1 = weight(a)
            val w2 = weight(b)
            when {
                w1 < w2 -> {
                    consStream(a) { mergeWeighted(s1.streamTail(), s2, weight) }
                }

                w1 > w2 -> {
                    consStream(b) { mergeWeighted(s1, s2.streamTail(), weight) }
                }

                else -> {
                    consStream(a) {
                        consStream(b) {
                            mergeWeighted(s1.streamTail(), s2.streamTail(), weight)
                        }
                    }
                }
            }
        }
    }
}

public fun weightedPairs(
    s: LStream<Long>,
    t: LStream<Long>,
    weight: (Pair<Long, Long>) -> Long,
): LStream<Pair<Long, Long>> {
    val sh = s.streamHead() ?: return LStream.Empty
    val th = t.streamHead() ?: return LStream.Empty
    return consStream(sh to th) {
        mergeWeighted(
            streamMap({ x -> sh to x }, t.streamTail()),
            weightedPairs(s.streamTail(), t.streamTail(), weight),
            weight,
        )
    }
}

/**
 * 3.70(b)'s component stream: the positive integers that 2, 3, and 5
 * fail to divide (1, 7, 11, 13, 17, ...). [weightedPairs] over it with
 * the weight `2i + 3j + 5ij` orders the pairs of 2, 3, 5-free numbers.
 */
public val integersNo235: LStream<Long> = streamFilter({ n -> n % 2L != 0L && n % 3L != 0L && n % 5L != 0L }, integers)
