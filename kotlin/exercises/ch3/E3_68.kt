// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.68

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * Exercise 3.68: Louis Reasoner thinks that building a stream of pairs
 * from three parts is unnecessarily complicated. Instead of separating
 * the pair `(S_0, T_0)` from the rest of the first row, he proposes to
 * work with the whole first row, interleaving it with the recursive
 * [louisPairs] of the tails. Does this work? Consider what happens if
 * we evaluate `louisPairs(integers, integers)`: the recursive call is an
 * argument to [interleave], so it runs before any consStream can fire,
 * and the construction diverges before the first element exists.
 */
public fun louisPairs(
    s: LStream<Long>,
    t: LStream<Long>,
): LStream<Pair<Long, Long>> = throw PendingSolution()
