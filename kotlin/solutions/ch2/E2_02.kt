// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.2

package sicp.ch2.exercises

public data class Point(
    val x: Double,
    val y: Double,
)

public fun makePoint(
    x: Double,
    y: Double,
): Point = Point(x, y)

public fun xPoint(p: Point): Double = p.x

public fun yPoint(p: Point): Double = p.y

public data class Segment(
    val startPoint: Point,
    val endPoint: Point,
)

public fun makeSegment(
    start: Point,
    end: Point,
): Segment = Segment(start, end)

public fun startSegment(s: Segment): Point = s.startPoint

public fun endSegment(s: Segment): Point = s.endPoint

private fun average(
    a: Double,
    b: Double,
): Double = (a + b) / 2.0

public fun midpointSegment(s: Segment): Point =
    makePoint(
        average(xPoint(startSegment(s)), xPoint(endSegment(s))),
        average(yPoint(startSegment(s)), yPoint(endSegment(s))),
    )

/** Matches the book's `print-point`: prints one line, returns no useful value. */
public fun printPoint(p: Point) {
    println("(${xPoint(p)},${yPoint(p)})")
}

public fun ex_2_02(): Point = midpointSegment(makeSegment(makePoint(2.0, 3.0), makePoint(8.0, 11.0)))
