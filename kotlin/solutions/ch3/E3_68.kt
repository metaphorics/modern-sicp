// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.68

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.streamHead
import sicp.runtime.streamTail

/**
 * Exercise 3.68: Louis Reasoner would build the stream of pairs from the
 * whole first row, without separating the diagonal pair off first. It
 * does not work. Scheme and Kotlin both evaluate an application's
 * arguments before the call, so the recursive [louisPairs] on the tails
 * runs before [interleave] can fire a single consStream: nothing ever
 * gets consed, the recursion has no base case, and constructing
 * `louisPairs(integers, integers)` diverges before even the first
 * element exists.
 */
public fun louisPairs(
    s: LStream<Long>,
    t: LStream<Long>,
): LStream<Pair<Long, Long>> {
    val sh = s.streamHead() ?: return LStream.Empty
    return interleave(
        streamMap({ x -> sh to x }, t),
        louisPairs(s.streamTail(), t.streamTail()),
    )
}
