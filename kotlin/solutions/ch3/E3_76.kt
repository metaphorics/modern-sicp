// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.76

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.streamTail

/**
 * Eva Lu Ator's `smooth`: each element of the result is the average of
 * two successive elements of [s], one delayed step over the input.
 */
public fun smooth(s: LStream<Double>): LStream<Double> = zipStream(s, s.streamTail()) { a, b -> (a + b) / 2.0 }

/**
 * The zero-crossing extractor as a reusable component: crossings of any
 * signal, seeded with 0 through the delayed copy. This is the modularity
 * the exercise asks for: conditioning the signal and extracting the
 * crossings are separate stages.
 */
public fun zeroCrossingsOf(signal: LStream<Double>): LStream<Long> = zipStream(signal, consStream(0.0) { signal }, ::signChangeDetector)

/**
 * Zero crossings of the smoothed [senseData]: the extractor is unchanged
 * from 3.74, the input is simply [smooth]ed first.
 */
public val smoothedZeroCrossings: LStream<Long> = zeroCrossingsOf(smooth(senseData))
