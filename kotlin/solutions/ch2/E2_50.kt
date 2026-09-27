// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.50

package sicp.ch2.exercises

/** Mirrors the image left-to-right: the new origin is the frame's top-right corner. */
public fun flipHoriz(painter: Painter): Painter =
    transformPainter(
        painter,
        makeVect(1.0, 0.0),
        makeVect(0.0, 0.0),
        makeVect(1.0, 1.0),
    )

/** Rotates the image counterclockwise by 180 degrees. */
public fun rotate180(painter: Painter): Painter =
    transformPainter(
        painter,
        makeVect(1.0, 1.0),
        makeVect(0.0, 1.0),
        makeVect(1.0, 0.0),
    )

/** Rotates the image counterclockwise by 270 degrees (equivalently, clockwise by 90 degrees). */
public fun rotate270(painter: Painter): Painter =
    transformPainter(
        painter,
        makeVect(0.0, 1.0),
        makeVect(0.0, 0.0),
        makeVect(1.0, 1.0),
    )

/** The segment count of `rotate180(wave)` painted into [unitSquare]. */
public fun ex_2_50(): Int = renderSegments(rotate180(wave), unitSquare).size
