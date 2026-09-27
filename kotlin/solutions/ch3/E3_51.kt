// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.51

package sicp.ch3.exercises

import sicp.runtime.LStream

/**
 * The mapped stream together with the log its recorder wrote. The book's
 * transcript becomes data: [log] holds one entry per [show] call, in the
 * order the pipeline ran them.
 */
public data class ShownStream(
    public val stream: LStream<Long>,
    public val log: MutableList<String>,
)

/**
 * Exercise 3.51: [show] returns its argument after displaying it, so the
 * transcript reveals when the delayed evaluation runs. Building the
 * mapped stream constructs its first cell, whose head already shows the
 * first element; [streamRef] then forces tails, and the memoized tail of
 * the book's `delay` runs each further [show] at most once. With
 * [memoized] false the twin builds its cells with an inline recorder and
 * no memoization: every fresh walk re-derives the cells it passes, so
 * the recorder runs again for each of them, the way a plain lambda
 * delay re-runs its expression on every force.
 */
public fun shownInterval(
    low: Long,
    high: Long,
    memoized: Boolean,
): ShownStream {
    val log = mutableListOf<String>()
    val stream =
        if (memoized) {
            streamMap({ show(it, log) }, streamEnumerateInterval(low, high))
        } else {
            coldShown(low, high, log)
        }
    return ShownStream(stream, log)
}

/** The unmemoized twin: the recorder runs at each cell's construction,
 * and each tail force builds a fresh successor. */
private fun coldShown(
    v: Long,
    high: Long,
    log: MutableList<String>,
): LStream<Long> =
    if (v > high) {
        LStream.Empty
    } else {
        LStream.Cons(show(v, log)) { coldShown(Math.addExact(v, 1L), high, log) }
    }
