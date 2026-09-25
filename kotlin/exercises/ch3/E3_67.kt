// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.67

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * Exercise 3.67: modify [pairs] so that `pairsAll(integers, integers)`
 * produces the stream of *all* pairs of integers `(i, j)`, without the
 * condition `i <= j`. Hint: you will need to mix in an additional
 * stream. The one here is the transposed row: each level conses the
 * diagonal pair, then interleaves the first row, its mirror column, and
 * the [pairsAll] of the tails.
 */
public fun pairsAll(
    s: LStream<Long>,
    t: LStream<Long>,
): LStream<Pair<Long, Long>> = throw PendingSolution()
