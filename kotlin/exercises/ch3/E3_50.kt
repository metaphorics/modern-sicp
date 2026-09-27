// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.50

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 3.50: complete the definition that generalizes the stream map
 * to procedures taking several arguments, the way the ordinary map of
 * section 2.2.1 does. The edition's n-stream map takes the procedure and
 * the list of streams and walks them in lockstep; the result runs out as
 * soon as any one stream does. Cold `Sequence` streams carry this
 * exercise: nothing here needs the memoized cons, and a second walk
 * recomputes every application.
 */
public fun <T, R> streamMapN(
    proc: (List<T>) -> R,
    streams: List<Sequence<T>>,
): Sequence<R> = throw PendingSolution()
