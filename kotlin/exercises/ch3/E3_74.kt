// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.74

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * Alyssa's sign-change detector: +1 when the signal moves from negative
 * to positive, -1 when it moves from positive to negative, 0 otherwise.
 * The sign of a 0 input is positive.
 */
public fun signChangeDetector(
    new: Double,
    old: Double,
): Long = throw PendingSolution()

/**
 * The book's recursive zero-crossing extractor: each element compares
 * the current input value with the previous one, seeded by [last].
 */
public fun makeZeroCrossings(
    input: LStream<Double>,
    last: Double,
): LStream<Long> = throw PendingSolution()

/** The sampled sensor signal from the exercise statement. */
public val senseData: LStream<Double>
    get() = throw PendingSolution()

/**
 * Eva Lu Ator's modular construction: map the detector over the signal
 * zipped with the same signal delayed by one element, the `<expression>`
 * the exercise asks for.
 */
public val zeroCrossings: LStream<Long>
    get() = throw PendingSolution()
