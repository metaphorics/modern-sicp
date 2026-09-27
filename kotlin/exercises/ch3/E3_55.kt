// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.55

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * Exercise 3.55: define partial-sums, taking a stream S and returning
 * the stream of S0, S0 + S1, S0 + S1 + S2, ... The result is its own
 * head followed by the elementwise sum of the partial sums with the
 * shifted input, so element k + 1 adds S(k + 1) to S's running total.
 * Kotlin locals cannot appear in their own initializer, so the tail
 * thunk reads the cell that the last line fills before anything can
 * force a tail.
 */
@JvmName("partialSumsL")
public fun partialSums(s: LStream<Long>): LStream<Long> = throw PendingSolution()

/** The same partial sums over real streams, for the pi series of
 * section 3.5.3. */
@JvmName("partialSumsR")
public fun partialSums(s: LStream<Double>): LStream<Double> = throw PendingSolution()
