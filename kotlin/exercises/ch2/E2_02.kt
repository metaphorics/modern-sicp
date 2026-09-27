// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.2

package sicp.ch2.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 2.2: consider the problem of representing line segments in a
 * plane. Each segment is represented as a pair of points: a starting point
 * and an ending point. Define a constructor `makeSegment` and selectors
 * `startSegment` and `endSegment` that define the representation of
 * segments in terms of points. Furthermore, a point can be represented as
 * a pair of numbers: the x coordinate and the y coordinate. Accordingly,
 * specify a constructor `makePoint` and selectors `xPoint` and `yPoint`
 * that define this representation. Finally, using your selectors and
 * constructors, define a procedure `midpointSegment` that takes a line
 * segment as an argument and returns its midpoint, the point whose
 * coordinates are the average of the coordinates of the endpoints. To try
 * your procedures, you will need a way to print points: `printPoint`. The
 * statement lives in the section 2.1 chapter text.
 *
 * The scaffold returns the midpoint of the segment from (2, 3) to (8, 11).
 */
public data class Point(
    val x: Double,
    val y: Double,
)

public fun ex_2_02(): Point = throw PendingSolution()
