// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.74

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.streamHead
import sicp.runtime.streamTail

/**
 * Alyssa's sign-change detector: +1 when the signal moves from negative
 * to positive, -1 when it moves from positive to negative, 0 otherwise.
 * The sign of 0 is positive, so a rise to exactly 0 counts as +1 and a
 * fall from 0 into the negative counts as -1.
 */
public fun signChangeDetector(
    new: Double,
    old: Double,
): Long {
    val newSign = if (new < 0.0) -1L else 1L
    val oldSign = if (old < 0.0) -1L else 1L
    return if (newSign == oldSign) 0L else newSign
}

/**
 * The book's recursive zero-crossing extractor: each element compares
 * the current input value with the previous one, seeded by [last]. The
 * previous value travels through the recursion as the raw signal.
 */
public fun makeZeroCrossings(
    input: LStream<Double>,
    last: Double,
): LStream<Long> {
    val head = input.streamHead() ?: return LStream.Empty
    return consStream(signChangeDetector(head, last)) { makeZeroCrossings(input.streamTail(), head) }
}

/** The sampled sensor signal from the exercise statement. */
public val senseData: LStream<Double> =
    listOf(1.0, 2.0, 1.5, 1.0, 0.5, -0.1, -2.0, -3.0, -2.0, -0.5, 0.2, 3.0, 4.0)
        .foldRight(LStream.Empty as LStream<Double>) { x, acc -> consStream(x) { acc } }

/**
 * Eva Lu Ator's modular construction: map the detector over the signal
 * zipped with the same signal delayed by one element and seeded with 0,
 * the `<expression>` the exercise asks for.
 */
public val zeroCrossings: LStream<Long> =
    zipStream(senseData, consStream(0.0) { senseData }, ::signChangeDetector)
