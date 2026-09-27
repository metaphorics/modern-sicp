// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.75

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.streamHead
import sicp.runtime.streamTail

/**
 * Louis Reasoner's version, reproduced for comparison: one state thread
 * has to serve two jobs. Because the recursion passes [lastValue] the
 * average, the raw previous input is lost after the first element: every
 * later average is taken against the previous AVERAGE, so the smoothing
 * drifts into an exponential average instead of Alyssa's plan of
 * averaging each value with the previous raw value. On a noisy signal
 * the corrupted average leaks the noise and reports crossings that are
 * not in the data.
 */
public fun makeZeroCrossingsBuggy(
    input: LStream<Double>,
    lastValue: Double,
): LStream<Long> {
    val head = input.streamHead() ?: return LStream.Empty
    val avpt = (head + lastValue) / 2.0
    return consStream(signChangeDetector(avpt, lastValue)) { makeZeroCrossingsBuggy(input.streamTail(), avpt) }
}

/**
 * The fix, without changing the structure: two state threads, exactly
 * Alyssa's plan. The previous AVERAGE ([lastAvg]) is the comparison
 * target; the raw previous input travels separately through the private
 * recursion, so each average pairs a raw value with the raw value before
 * it. [lastAvg] also seeds the raw thread, which reads the signal as
 * having been at rest at that value before the first sample.
 */
public fun makeZeroCrossingsSmoothed(
    input: LStream<Double>,
    lastAvg: Double,
): LStream<Long> = smoothedCrossings(input, lastAvg, lastAvg)

/** The fixed extractor's recursion: [lastValue] is the raw previous
 * input, [lastAvg] the previous average the detector compares against. */
private fun smoothedCrossings(
    input: LStream<Double>,
    lastValue: Double,
    lastAvg: Double,
): LStream<Long> {
    val head = input.streamHead() ?: return LStream.Empty
    val avpt = (head + lastValue) / 2.0
    return consStream(signChangeDetector(avpt, lastAvg)) { smoothedCrossings(input.streamTail(), head, avpt) }
}
