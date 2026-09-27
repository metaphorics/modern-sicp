// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.51

package sicp.ch2.exercises

/**
 * First way: analogous to [beside], with the split point on the vertical
 * edge instead of the horizontal one. This is exactly the construction
 * the shared `Painters.kt` carries as `below`, restated here as this
 * exercise's own answer.
 */
public fun belowViaTransform(
    painter1: Painter,
    painter2: Painter,
): Painter {
    val splitPoint = makeVect(0.0, 0.5)
    val paintBottom =
        transformPainter(painter1, makeVect(0.0, 0.0), makeVect(1.0, 0.0), splitPoint)
    val paintTop =
        transformPainter(painter2, splitPoint, makeVect(1.0, 0.5), makeVect(0.0, 1.0))
    return { frame, out ->
        paintBottom(frame, out)
        paintTop(frame, out)
    }
}

/**
 * Second way: in terms of [beside] and the rotations of exercise 2.50.
 * Rotating both painters 270 degrees turns "left/right" into
 * "bottom/top"; placing them beside each other and rotating the whole
 * assembly 90 degrees undoes the per-painter rotation while leaving the
 * bottom/top placement in effect.
 */
public fun belowViaRotation(
    painter1: Painter,
    painter2: Painter,
): Painter = rotate90(beside(rotate270(painter1), rotate270(painter2)))

/** Whether the two constructions paint the same segments for a sample pair. */
public fun ex_2_51(): Boolean =
    renderSegments(belowViaTransform(wave, xPainter), unitSquare) ==
        renderSegments(belowViaRotation(wave, xPainter), unitSquare)
