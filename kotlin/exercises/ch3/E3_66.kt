// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.66

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 3.66: examine the stream [pairs] makes of [integers] and say
 * what order the pairs arrive in. Can you make general comments about
 * the order in which the pairs are placed into the stream? For example,
 * about how many pairs precede the pair (1, 100)? The pair (99, 100)?
 * The pair (100, 100)?
 *
 * The returned index is 0-based: the number of pairs preceding the one
 * asked about. The laws are `(1, j)` at `2j - 3`, the diagonal `(k, k)`
 * at `2^k - 2`, and the upper triangle `(i, j)` with `1 < i < j` at
 * `2^i (j - i) + 2^(i-1) - 2`.
 */
public fun positionOf(pair: Pair<Long, Long>): Long? = throw PendingSolution()
