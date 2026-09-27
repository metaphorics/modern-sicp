// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.71

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * Exercise 3.71: numbers that can be expressed as the sum of two cubes
 * in more than one way are sometimes called Ramanujan numbers. To find
 * them, generate the stream of pairs of integers `(i, j)` weighted
 * according to `i^3 + j^3` (see [weightedPairs] of 3.70), then search
 * the stream for runs of consecutive pairs with the same weight. The
 * first such number is 1729. [ramanujanNumbers] emits the weights that
 * two or more consecutive pairs share; the next five after 1729 are
 * 4104, 13832, 20683, 32832, and 39312.
 */
public fun ramanujanNumbers(): LStream<Long> = throw PendingSolution()
