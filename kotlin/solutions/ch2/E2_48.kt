// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.48

package sicp.ch2.exercises

import kotlin.math.sqrt

/**
 * `Segment`, `makeSegment`, `startSegment`, and `endSegment`, all
 * declared in this package's shared `Painters.kt`, are exactly this
 * exercise's answer, carried early for the same reason `Vect` is: the
 * primitive `wave` painter needs a working segment representation before
 * the exercise order reaches this file. This file demonstrates the
 * abstraction directly with a length computed only from the two vector
 * selectors.
 */
public fun segmentLength(segment: Segment): Double {
    val dx = xcorVect(endSegment(segment)) - xcorVect(startSegment(segment))
    val dy = ycorVect(endSegment(segment)) - ycorVect(startSegment(segment))
    return sqrt(dx * dx + dy * dy)
}

/** The length of the segment from (0, 0) to (3, 4). */
public fun ex_2_48(): Double = segmentLength(makeSegment(makeVect(0.0, 0.0), makeVect(3.0, 4.0)))
