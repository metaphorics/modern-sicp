// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.take

/**
 * The cold pipeline: the generator loop runs once per terminal operation,
 * so a second `take` recomputes everything.
 */
public fun squaresSeq(force: () -> Unit): Sequence<Long> =
    sequence {
        var n = 1L
        while (true) {
            force()
            yield(n * n)
            n += 1L
        }
    }

/** One node materialization; the recursion supplies the memoized tail. */
private fun squaresFrom(
    n: Long,
    force: () -> Unit,
): LStream<Long> {
    force()
    return consStream(n * n) { squaresFrom(n + 1L, force) }
}

/** The memoized stream: each node computes once, however many walks read it. */
public fun squaresStream(force: () -> Unit): LStream<Long> = squaresFrom(1L, force)

/** Builds one of each, walks each twice with `take(5)`, counts the forces.
 * `take(5)` forces one tail beyond the fifth head, so the stream counts 6. */
public fun ex_0_05(): Pair<Int, Int> {
    var seqForces = 0
    val seq = squaresSeq { seqForces += 1 }
    seq.take(5).toList()
    seq.take(5).toList()
    var streamForces = 0
    val stream = squaresStream { streamForces += 1 }
    stream.take(5)
    stream.take(5)
    return seqForces to streamForces
}
