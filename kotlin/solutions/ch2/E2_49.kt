// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.49

package sicp.ch2.exercises

/** (a) The outline of the designated frame: its four sides. */
public val outlinePainter: Painter =
    segmentsToPainter(
        listOf(
            makeSegment(makeVect(0.0, 0.0), makeVect(1.0, 0.0)),
            makeSegment(makeVect(1.0, 0.0), makeVect(1.0, 1.0)),
            makeSegment(makeVect(1.0, 1.0), makeVect(0.0, 1.0)),
            makeSegment(makeVect(0.0, 1.0), makeVect(0.0, 0.0)),
        ),
    )

/** (b) An "X" connecting opposite corners of the frame. */
public val xPainter: Painter =
    segmentsToPainter(
        listOf(
            makeSegment(makeVect(0.0, 0.0), makeVect(1.0, 1.0)),
            makeSegment(makeVect(0.0, 1.0), makeVect(1.0, 0.0)),
        ),
    )

/** (c) A diamond connecting the midpoints of the frame's sides. */
public val diamondPainter: Painter =
    segmentsToPainter(
        listOf(
            makeSegment(makeVect(0.5, 0.0), makeVect(1.0, 0.5)),
            makeSegment(makeVect(1.0, 0.5), makeVect(0.5, 1.0)),
            makeSegment(makeVect(0.5, 1.0), makeVect(0.0, 0.5)),
            makeSegment(makeVect(0.0, 0.5), makeVect(0.5, 0.0)),
        ),
    )

// (d) The wave painter is `wave`, declared in the shared `Painters.kt`, also built with segmentsToPainter.

/** The segment counts of the three painters defined above, painted into [unitSquare]. */
public fun ex_2_49(): List<Int> = listOf(outlinePainter, xPainter, diamondPainter).map { renderSegments(it, unitSquare).size }
