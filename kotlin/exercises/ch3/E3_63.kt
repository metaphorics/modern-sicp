// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.63

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/** Counts sqrt-improve invocations so a test can price each variant. */
public class SqrtProbe {
    public var improveCalls: Int = 0
}

/**
 * Exercise 3.63, Alyssa's version: the book's sqrt-stream with the
 * local guesses stream, each improve step counted in [probe].
 */
public fun sqrtStreamShared(
    x: Double,
    probe: SqrtProbe,
): LStream<Double> = throw PendingSolution()

/**
 * Louis Reasoner's version without the local guesses stream: each tail
 * calls the whole procedure again, each improve step counted in
 * [probe].
 */
public fun sqrtStreamFresh(
    x: Double,
    probe: SqrtProbe,
): LStream<Double> = throw PendingSolution()
