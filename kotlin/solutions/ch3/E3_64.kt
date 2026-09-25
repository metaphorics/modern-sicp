// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.64

package sicp.ch3.exercises

import sicp.runtime.LStream

/**
 * Exercise 3.64: walk [s] until two successive elements differ by less
 * than [tolerance] in absolute value, and return the second of the
 * pair. A stream that runs dry first has not converged, which is an
 * error rather than a value.
 */
public fun streamLimit(
    s: LStream<Double>,
    tolerance: Double,
): Double {
    var cursor = s
    while (cursor is LStream.Cons) {
        val next = cursor.tail
        if (next is LStream.Cons && kotlin.math.abs(cursor.head - next.head) < tolerance) {
            return next.head
        }
        cursor = next
    }
    throw IllegalStateException("stream-limit: no two successive elements came within the tolerance")
}

/** Square roots up to a tolerance: the limit of the guess stream. */
public fun sqrtByLimit(
    x: Double,
    tolerance: Double,
): Double = streamLimit(sqrtStream(x), tolerance)
