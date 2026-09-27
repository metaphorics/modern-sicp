// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.52

package sicp.ch2.exercises

/** (a) The primitive wave with two extra segments added: a small smile. */
public val waveSegmentsWithSmile: List<Segment> =
    waveSegments +
        listOf(
            makeSegment(makeVect(0.4375, 0.4375), makeVect(0.5, 0.375)),
            makeSegment(makeVect(0.5, 0.375), makeVect(0.5625, 0.4375)),
        )

/** The wave-with-a-smile painter. */
public val waveWithSmile: Painter = segmentsToPainter(waveSegmentsWithSmile)

/**
 * (b) `corner-split`, changed to use only one copy of the up-split and
 * right-split images instead of two: `topLeft` is `up`, not
 * `beside(up, up)`, and `bottomRight` is `right`, not `below(right,
 * right)`.
 */
public fun cornerSplitModified(
    painter: Painter,
    n: Int,
): Painter =
    if (n == 0) {
        painter
    } else {
        val up = upSplit(painter, n - 1)
        val right = rightSplit(painter, n - 1)
        val corner = cornerSplitModified(painter, n - 1)
        beside(below(painter, up), below(right, corner))
    }

/**
 * (c) `square-limit`, using `square-of-four` to assemble the four
 * corners in a different pattern: the book's arrangement is `(flip-horiz
 * identity rotate180 flip-vert)`; here the corners face outward from the
 * center instead, `(rotate180 flip-vert flip-horiz identity)`.
 */
public fun squareLimitModified(
    painter: Painter,
    n: Int,
): Painter {
    val combine4 = squareOfFour(::rotate180, ::flipVert, ::flipHoriz, { p -> p })
    return combine4(cornerSplitModified(painter, n))
}

/** The segment count of `squareLimitModified(waveWithSmile, 1)` painted into [unitSquare]. */
public fun ex_2_52(): Int = renderSegments(squareLimitModified(waveWithSmile, 1), unitSquare).size
