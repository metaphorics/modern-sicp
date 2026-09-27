// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.66

package sicp.ch3.exercises

import sicp.runtime.LStream

/**
 * Exercise 3.66: examine the stream [pairs] makes of [integers] and say
 * what order the pairs arrive in. The indices below are 0-based: the
 * number of pairs that precede the one asked about.
 *
 * - The first row sits at the even offsets: `(1, j)` at `2j - 3`, so
 *   (1, 100) is preceded by 197 pairs.
 * - The diagonal `(k, k)` opens the recursive remainder at `2^k - 2`, so
 *   (100, 100) is preceded by `2^100 - 2` pairs.
 * - The rest of the upper triangle `(i, j)` with `1 < i < j` sits at
 *   `2^i (j - i) + 2^(i-1) - 2`, so (99, 100) is preceded by
 *   `2^99 + 2^98 - 2 = 950737950171172051122527404030` pairs.
 *
 * [positionOf] walks the stream and returns such an index, or null when
 * the walk gives up at [SEARCH_LIMIT] elements; (99, 100) is far past
 * any walk and is asserted from the law instead.
 */
public fun positionOf(pair: Pair<Long, Long>): Long? {
    var cursor: LStream<Pair<Long, Long>> = pairs(integers, integers)
    var index = 0L
    while (index < SEARCH_LIMIT && cursor is LStream.Cons) {
        if (cursor.head == pair) {
            return index
        }
        index++
        cursor = cursor.tail
    }
    return null
}

/** The walk's give-up point; every walked pin sits inside it. */
private const val SEARCH_LIMIT: Long = 65536L
