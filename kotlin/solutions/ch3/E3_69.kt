// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.69

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.streamHead
import sicp.runtime.streamTail

/**
 * Exercise 3.69: [triples] takes three infinite streams and produces the
 * stream of triples `(S_i, T_j, U_k)` with `i <= j <= k`. Each level
 * conses the diagonal triple, then interleaves the first component
 * mapped over the [pairs] of the tails (whose entries already satisfy
 * `j <= k`) with the [triples] of the tails.
 */
public fun triples(
    s: LStream<Long>,
    t: LStream<Long>,
    u: LStream<Long>,
): LStream<Triple<Long, Long, Long>> {
    val sh = s.streamHead() ?: return LStream.Empty
    val th = t.streamHead() ?: return LStream.Empty
    val uh = u.streamHead() ?: return LStream.Empty
    return consStream(Triple(sh, th, uh)) {
        interleave(
            streamMap({ p -> Triple(sh, p.first, p.second) }, pairs(t.streamTail(), u.streamTail())),
            triples(s.streamTail(), t.streamTail(), u.streamTail()),
        )
    }
}

/**
 * The Pythagorean triples of positive integers: [triples] over the
 * integers, filtered to the triples with `i^2 + j^2 = k^2`. The first
 * six, in stream order, are (3, 4, 5), (6, 8, 10), (5, 12, 13),
 * (9, 12, 15), (8, 15, 17), and (12, 16, 20).
 */
public fun pythagoreanTriples(): LStream<Triple<Long, Long, Long>> = filterPythagorean(triples(integers, integers, integers))

private fun isPythagorean(t: Triple<Long, Long, Long>): Boolean = t.first * t.first + t.second * t.second == t.third * t.third

/** Skips misses with a cursor loop instead of the recursive
 * [streamFilter] shape: Kotlin has no tail calls, and one JVM frame per
 * miss overflows long stretches (the sixth triple sits almost 300,000
 * triples into the stream). */
private fun filterPythagorean(s: LStream<Triple<Long, Long, Long>>): LStream<Triple<Long, Long, Long>> {
    var cursor = s
    while (cursor is LStream.Cons) {
        if (isPythagorean(cursor.head)) {
            val hit = cursor
            return consStream(hit.head) { filterPythagorean(hit.tail) }
        }
        cursor = cursor.tail
    }
    return LStream.Empty
}
