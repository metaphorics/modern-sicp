// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.3

package sicp.ch2.exercises

import kotlin.math.abs

// Point, makePoint, xPoint, and yPoint are exercise 2.2's public declarations, reused here from the same package.

/** Representation A: one corner plus a width and a height. */
public data class RectangleCornerSize(
    val corner: Point,
    val rectWidth: Double,
    val rectHeight: Double,
)

public fun makeRectangleCornerSize(
    corner: Point,
    width: Double,
    height: Double,
): RectangleCornerSize = RectangleCornerSize(corner, width, height)

/** Representation B: two opposite corners. */
public data class RectangleCorners(
    val bottomLeft: Point,
    val topRight: Point,
)

public fun makeRectangleCorners(
    bottomLeft: Point,
    topRight: Point,
): RectangleCorners = RectangleCorners(bottomLeft, topRight)

/**
 * The abstraction barrier: [perimeter] and [area] are written once, over a
 * width and a height, and never see which representation produced them.
 */
public fun perimeter(
    width: Double,
    height: Double,
): Double = 2.0 * (width + height)

public fun area(
    width: Double,
    height: Double,
): Double = width * height

public fun ex_2_03(): Pair<Double, Double> {
    val a = makeRectangleCornerSize(makePoint(0.0, 0.0), 4.0, 3.0)
    val aWidth = a.rectWidth
    val aHeight = a.rectHeight

    val b = makeRectangleCorners(makePoint(0.0, 0.0), makePoint(4.0, 3.0))
    val bWidth = abs(xPoint(b.topRight) - xPoint(b.bottomLeft))
    val bHeight = abs(yPoint(b.topRight) - yPoint(b.bottomLeft))

    check(perimeter(aWidth, aHeight) == perimeter(bWidth, bHeight))
    check(area(aWidth, aHeight) == area(bWidth, bHeight))
    return perimeter(aWidth, aHeight) to area(aWidth, aHeight)
}
