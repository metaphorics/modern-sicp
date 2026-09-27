// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.69

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * Exercise 3.69: write [triples], which takes three infinite streams and
 * produces the stream of triples `(S_i, T_j, U_k)` such that
 * `i <= j <= k`. Use [triples] to generate the stream of all Pythagorean
 * triples of positive integers, the triples with `i <= j` and
 * `i^2 + j^2 = k^2`.
 */
public fun triples(
    s: LStream<Long>,
    t: LStream<Long>,
    u: LStream<Long>,
): LStream<Triple<Long, Long, Long>> = throw PendingSolution()

/** The Pythagorean stream over [triples] of the integers; the first six
 * are (3, 4, 5), (6, 8, 10), (5, 12, 13), (9, 12, 15), (8, 15, 17), and
 * (12, 16, 20). */
public fun pythagoreanTriples(): LStream<Triple<Long, Long, Long>> = throw PendingSolution()
