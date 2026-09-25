// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.60

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.streamHead
import sicp.runtime.streamTail

/**
 * Exercise 3.60: the product of two power series. The constant term is
 * the product of the two constant terms, and the rest adds the second
 * series' tail scaled by the first series' constant term to the
 * product of the two tails: (c1 + S1')(c2 + S2') = c1 c2 + (c2 S1' +
 * S1' S2' + c1 S2') grouped as c1 c2 + c2 S1' + S1'(c2 + S2'). The
 * self-referential call in the tail is what the book's
 * `cons-stream`/`add-streams` shape buys.
 */
public fun mulSeries(
    s1: LStream<Rat>,
    s2: LStream<Rat>,
): LStream<Rat> {
    val c1 = s1.streamHead() ?: return LStream.Empty
    val c2 = s2.streamHead() ?: return LStream.Empty
    return consStream(c1 * c2) {
        zipStream(
            streamMap({ it * c1 }, s2.streamTail()),
            mulSeries(s1.streamTail(), s2),
        ) { a, b -> a + b }
    }
}
