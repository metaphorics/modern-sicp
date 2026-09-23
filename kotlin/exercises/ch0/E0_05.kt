// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * Exercise 0.5: the infinite `Sequence` of squares and the memoized
 * `LStream` of squares, then a counting probe.
 *
 * Section 0.3.
 *
 * [squaresSeq] yields 1, 4, 9, ... forever; each element the pipeline
 * computes calls [force] once. [squaresStream] is the runtime's memoized
 * stream of the same values; each node materialization calls [force] once.
 * [ex_0_05] builds one of each, walks each one twice with `take(5)`, and
 * returns (sequence forces, stream forces). Note `take(5)` forces one tail
 * beyond the fifth head: the stream count is 6, not 5. The statement lives
 * in the section 0.3 chapter text.
 */
public fun squaresSeq(force: () -> Unit): Sequence<Long> = throw PendingSolution()

public fun squaresStream(force: () -> Unit): LStream<Long> = throw PendingSolution()

public fun ex_0_05(): Pair<Int, Int> = throw PendingSolution()
