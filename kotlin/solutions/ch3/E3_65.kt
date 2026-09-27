// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.65

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.streamHead
import sicp.runtime.streamTail

// Exercise 3.65: ln 2 = 1 - 1/2 + 1/3 - 1/4 + ... as three
// approximation streams. The raw partial sums converge slowly (the
// error is still a few hundredths after ten terms), the
// Euler-transformed series buys about two digits per element, and the
// full tableau of repeated transforms converges to machine precision
// within a handful of elements.

/** The ln 2 summands 1, -1/2, 1/3, -1/4, ... */
private val lnSummands: LStream<Double> =
    streamMap({ n -> if (n % 2L == 1L) 1.0 / n else -1.0 / n }, integersStartingFrom(1L))

/** The running sums, this file's own partial sums (3.55's public answer is
 * not imported: the exercise is answered on its own). */
private fun lnPartialSums(s: LStream<Double>): LStream<Double> {
    val head = s.streamHead() ?: return LStream.Empty
    var sums: LStream<Double>? = null
    val tied: LStream<Double> =
        consStream(head) { addStreams(checkNotNull(sums) { "sums not yet tied" }, s.streamTail()) }
    sums = tied
    return tied
}

/** The raw partial sums of the ln 2 series. */
public val ln2Raw: LStream<Double> = lnPartialSums(lnSummands)

/** The Euler transform of the raw sums: one acceleration step per element. */
public val ln2Euler: LStream<Double> = eulerTransform(ln2Raw)

/** The tableau of repeated Euler transforms, read down its first column. */
public val ln2Accelerated: LStream<Double> = acceleratedSequence(::eulerTransform, ln2Raw)
