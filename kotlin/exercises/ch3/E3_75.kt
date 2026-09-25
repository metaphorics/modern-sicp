// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.75

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * Louis Reasoner's version of Alyssa's smoothing plan, reproduced: one
 * state thread serves both jobs, so the raw previous input is lost after
 * the first element and the smoothing drifts into an exponential
 * average. This is the buggy program the exercise asks to find.
 */
public fun makeZeroCrossingsBuggy(
    input: LStream<Double>,
    lastValue: Double,
): LStream<Long> = throw PendingSolution()

/**
 * The fix, without changing the structure: [lastAvg] is the previous
 * AVERAGE and the comparison target, while the raw previous input
 * travels through the recursion separately, so each average pairs raw
 * values. [lastAvg] also seeds the raw thread.
 */
public fun makeZeroCrossingsSmoothed(
    input: LStream<Double>,
    lastAvg: Double,
): LStream<Long> = throw PendingSolution()
