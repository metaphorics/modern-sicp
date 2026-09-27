// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.67

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.streamHead
import sicp.runtime.streamTail

/**
 * Exercise 3.67: modify [pairs] so that `pairsAll(integers, integers)`
 * produces the stream of *all* pairs of integers `(i, j)`, without the
 * condition `i <= j`. The hint's additional stream is the transposed
 * row: each level conses the diagonal pair, then interleaves the first
 * row `(S_0, T_1), (S_0, T_2), ...`, its mirror column
 * `(S_1, T_0), (S_2, T_0), ...`, and the [pairsAll] of the tails.
 */
public fun pairsAll(
    s: LStream<Long>,
    t: LStream<Long>,
): LStream<Pair<Long, Long>> {
    val sh = s.streamHead() ?: return LStream.Empty
    val th = t.streamHead() ?: return LStream.Empty
    return consStream(sh to th) {
        interleave(
            streamMap({ x -> sh to x }, t.streamTail()),
            interleave(
                streamMap({ x -> x to th }, s.streamTail()),
                pairsAll(s.streamTail(), t.streamTail()),
            ),
        )
    }
}
