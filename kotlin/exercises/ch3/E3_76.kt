// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.76

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * Eva Lu Ator's `smooth`: each element of the result is the average of
 * two successive elements of [s].
 */
public fun smooth(s: LStream<Double>): LStream<Double> = throw PendingSolution()

/**
 * The zero-crossing extractor as a reusable component: crossings of any
 * [signal], seeded with 0 through the delayed copy.
 */
public fun zeroCrossingsOf(signal: LStream<Double>): LStream<Long> = throw PendingSolution()

/**
 * Zero crossings of the smoothed sensor signal: smoothing first, then
 * the unchanged extractor.
 */
public val smoothedZeroCrossings: LStream<Long>
    get() = throw PendingSolution()
