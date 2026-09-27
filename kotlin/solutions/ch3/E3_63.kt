// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.63

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream

/** Counts sqrt-improve invocations so a test can price each variant. */
public class SqrtProbe {
    public var improveCalls: Int = 0
}

/** The book's sqrt-improve step with the probe counting each call; both
 * variants below differ only in how they feed this one step. */
private fun probingImprove(
    probe: SqrtProbe,
    x: Double,
): (Double) -> Double =
    { g ->
        probe.improveCalls += 1
        sqrtImprove(g, x)
    }

/**
 * Exercise 3.63, Alyssa's version: the book's sqrt-stream with the
 * local guesses stream. The map's delay is memoized, so each improve
 * step runs once per node no matter how many times the stream is
 * walked or how many cursors share it.
 */
public fun sqrtStreamShared(
    x: Double,
    probe: SqrtProbe,
): LStream<Double> {
    var guesses: LStream<Double>? = null
    val improve = probingImprove(probe, x)
    val tied: LStream<Double> =
        consStream(1.0) { streamMap(improve, checkNotNull(guesses) { "guesses not yet tied" }) }
    guesses = tied
    return tied
}

/**
 * Louis Reasoner's version: every tail calls the whole procedure again,
 * so reaching element n rebuilds a fresh n-1-long improvement cascade.
 * The probe shows the redundant work the local guesses stream avoids.
 */
public fun sqrtStreamFresh(
    x: Double,
    probe: SqrtProbe,
): LStream<Double> {
    val improve = probingImprove(probe, x)
    return consStream(1.0) { streamMap(improve, sqrtStreamFresh(x, probe)) }
}
