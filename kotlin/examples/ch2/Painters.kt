// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.2.4: the picture language's given machinery

package sicp.ch2.examples

// The picture language's given machinery in one place: vectors (the
// subject of exercise 2.46), frames (2.47), segments (2.48), the segment
// painter (2.49), `transformPainter` with `flipVert` and `rotate90`, the
// two combinators `beside` and `below`, and the recursive plans
// `rightSplit`, `cornerSplit`, and `squareLimit` of the prose. A painter
// draws by appending SVG line elements to a StringBuilder, so a picture
// is a plain string. Exercises 2.46 to 2.52 re-derive named pieces
// of this file and prove them equivalent; each exercise's rationale says
// which piece is carried early and why.

/** A two-dimensional vector: the `make-vect` data abstraction of 2.46. */
public data class Vect(
    val x: Double,
    val y: Double,
)

/** The book's `make-vect`. */
public fun makeVect(
    x: Double,
    y: Double,
): Vect = Vect(x, y)

/** The book's `xcor-vect`. */
public fun xcorVect(v: Vect): Double = v.x

/** The book's `ycor-vect`. */
public fun ycorVect(v: Vect): Double = v.y

/** The book's `add-vect`. */
public fun addVect(
    a: Vect,
    b: Vect,
): Vect = Vect(a.x + b.x, a.y + b.y)

/** The book's `sub-vect`. */
public fun subVect(
    a: Vect,
    b: Vect,
): Vect = Vect(a.x - b.x, a.y - b.y)

/** The book's `scale-vect`. */
public fun scaleVect(
    a: Vect,
    s: Double,
): Vect = Vect(a.x * s, a.y * s)

/** A frame: an origin vector and two edge vectors. */
public data class Frame(
    val origin: Vect,
    val edge1: Vect,
    val edge2: Vect,
)

/** The book's `make-frame`; exercise 2.47 derives selector pairs for it. */
public fun makeFrame(
    origin: Vect,
    edge1: Vect,
    edge2: Vect,
): Frame = Frame(origin, edge1, edge2)

/** The book's `origin-frame`. */
public fun originFrame(frame: Frame): Vect = frame.origin

/** The book's `edge1-frame`. */
public fun edge1Frame(frame: Frame): Vect = frame.edge1

/** The book's `edge2-frame`. */
public fun edge2Frame(frame: Frame): Vect = frame.edge2

/** A directed line segment between two vectors (exercise 2.48). */
public data class Segment(
    val start: Vect,
    val end: Vect,
)

/** The book's `make-segment`. */
public fun makeSegment(
    start: Vect,
    end: Vect,
): Segment = Segment(start, end)

/** The book's `start-segment`. */
public fun startSegment(segment: Segment): Vect = segment.start

/** The book's `end-segment`. */
public fun endSegment(segment: Segment): Vect = segment.end

/**
 * A painter draws its image shifted and scaled into a frame by appending
 * SVG line elements to [out]. The book's painters are procedures from a
 * frame to a drawing; the drawing buffer is an explicit argument so a
 * picture is a plain string.
 */
public typealias Painter = (Frame, StringBuilder) -> Unit

/** The unit-square frame the tests and figures draw into. */
public val unitSquare: Frame =
    Frame(makeVect(0.0, 0.0), makeVect(1.0, 0.0), makeVect(0.0, 1.0))

/**
 * The frame coordinate map: it shifts and scales the unit square into the
 * frame, mapping `(0, 0)` to the frame's origin and `(1, 1)` to the vertex
 * diagonally opposite it.
 */
public fun frameCoordMap(frame: Frame): (Vect) -> Vect =
    { v ->
        addVect(
            originFrame(frame),
            addVect(
                scaleVect(edge1Frame(frame), xcorVect(v)),
                scaleVect(edge2Frame(frame), ycorVect(v)),
            ),
        )
    }

/** The book's `draw-line` primitive: appends one SVG line element. */
public fun drawLine(
    out: StringBuilder,
    from: Vect,
    to: Vect,
) {
    out.append("<line x1=\"${from.x}\" y1=\"${from.y}\" x2=\"${to.x}\" y2=\"${to.y}\"/>")
}

/** The book's `segments->painter`, drawing each transformed segment. */
public fun segmentsToPainter(segmentList: List<Segment>): Painter =
    { frame, out ->
        val map = frameCoordMap(frame)
        for (segment in segmentList) {
            drawLine(out, map(startSegment(segment)), map(endSegment(segment)))
        }
    }

/** A crude line drawing in the unit square, on a 1/8 grid so every transformed coordinate stays exact. */
public val waveSegments: List<Segment> =
    listOf(
        makeSegment(makeVect(0.0, 0.625), makeVect(0.25, 0.5)),
        makeSegment(makeVect(0.25, 0.5), makeVect(0.375, 0.625)),
        makeSegment(makeVect(1.0, 0.625), makeVect(0.75, 0.5)),
        makeSegment(makeVect(0.75, 0.5), makeVect(0.625, 0.625)),
        makeSegment(makeVect(0.375, 0.625), makeVect(0.5, 0.5)),
        makeSegment(makeVect(0.5, 0.5), makeVect(0.625, 0.625)),
        makeSegment(makeVect(0.375, 0.625), makeVect(0.375, 0.375)),
        makeSegment(makeVect(0.625, 0.625), makeVect(0.625, 0.375)),
        makeSegment(makeVect(0.375, 0.375), makeVect(0.5, 0.25)),
        makeSegment(makeVect(0.5, 0.25), makeVect(0.625, 0.375)),
        makeSegment(makeVect(0.5, 0.25), makeVect(0.5, 0.0)),
        makeSegment(makeVect(0.375, 0.375), makeVect(0.25, 0.0)),
        makeSegment(makeVect(0.625, 0.375), makeVect(0.75, 0.0)),
        makeSegment(makeVect(0.375, 0.375), makeVect(0.625, 0.375)),
    )

/** The section's primitive `wave` painter. */
public val wave: Painter = segmentsToPainter(waveSegments)

/**
 * The book's `transform-painter`: builds a painter that calls the original
 * one on a frame whose origin is the image of [origin] and whose edge
 * vectors end at the images of [corner1] and [corner2].
 */
public fun transformPainter(
    painter: Painter,
    origin: Vect,
    corner1: Vect,
    corner2: Vect,
): Painter =
    { frame, out ->
        val map = frameCoordMap(frame)
        val newOrigin = map(origin)
        painter(
            Frame(
                newOrigin,
                subVect(map(corner1), newOrigin),
                subVect(map(corner2), newOrigin),
            ),
            out,
        )
    }

/** The prose's vertical flip: the new origin is the top-left corner. */
public fun flipVert(painter: Painter): Painter =
    transformPainter(
        painter,
        makeVect(0.0, 1.0),
        makeVect(1.0, 1.0),
        makeVect(0.0, 0.0),
    )

/** The prose's counterclockwise 90 degree rotation. */
public fun rotate90(painter: Painter): Painter =
    transformPainter(
        painter,
        makeVect(1.0, 0.0),
        makeVect(1.0, 1.0),
        makeVect(0.0, 0.0),
    )

/** The prose's `beside`: the first painter left, the second right. */
public fun beside(
    painter1: Painter,
    painter2: Painter,
): Painter {
    val splitPoint = makeVect(0.5, 0.0)
    val paintLeft =
        transformPainter(
            painter1,
            makeVect(0.0, 0.0),
            splitPoint,
            makeVect(0.0, 1.0),
        )
    val paintRight =
        transformPainter(
            painter2,
            splitPoint,
            makeVect(1.0, 0.0),
            makeVect(0.5, 1.0),
        )
    return { frame, out ->
        paintLeft(frame, out)
        paintRight(frame, out)
    }
}

/**
 * The prose's `below`: the first painter in the bottom of the frame, the
 * second in the top. The prose gives the behavior from `wave4` on and
 * leaves the construction to exercise 2.51; this file carries the
 * transform-painter construction so the earlier listings run.
 */
public fun below(
    painter1: Painter,
    painter2: Painter,
): Painter {
    val splitPoint = makeVect(0.0, 0.5)
    val paintBottom =
        transformPainter(
            painter1,
            makeVect(0.0, 0.0),
            makeVect(1.0, 0.0),
            splitPoint,
        )
    val paintTop =
        transformPainter(
            painter2,
            splitPoint,
            makeVect(1.0, 0.5),
            makeVect(0.0, 1.0),
        )
    return { frame, out ->
        paintBottom(frame, out)
        paintTop(frame, out)
    }
}

/**
 * The book's `up-split`, branched upwards. Exercise 2.44 derives it; this
 * file carries a copy under a given-only name so `cornerSplit` can run
 * before the exercise is reached, and 2.44's solution proves its own
 * `upSplit` equivalent to this one.
 */
private fun upSplitGiven(
    painter: Painter,
    n: Int,
): Painter =
    if (n == 0) {
        painter
    } else {
        val smaller = upSplitGiven(painter, n - 1)
        below(painter, beside(smaller, smaller))
    }

/** The prose's `right-split`: branch to the right. */
public fun rightSplit(
    painter: Painter,
    n: Int,
): Painter =
    if (n == 0) {
        painter
    } else {
        val smaller = rightSplit(painter, n - 1)
        beside(painter, below(smaller, smaller))
    }

/** The prose's `corner-split`: branch up and to the right. */
public fun cornerSplit(
    painter: Painter,
    n: Int,
): Painter =
    if (n == 0) {
        painter
    } else {
        val up = upSplitGiven(painter, n - 1)
        val right = rightSplit(painter, n - 1)
        val topLeft = beside(up, up)
        val bottomRight = below(right, right)
        val corner = cornerSplit(painter, n - 1)
        beside(below(painter, topLeft), below(bottomRight, corner))
    }

/**
 * The prose's `square-of-four`: four one-argument painter operations for
 * the top-left, top-right, bottom-left, and bottom-right copies.
 */
public fun squareOfFour(
    tl: (Painter) -> Painter,
    tr: (Painter) -> Painter,
    bl: (Painter) -> Painter,
    br: (Painter) -> Painter,
): (Painter) -> Painter =
    { painter ->
        val top = beside(tl(painter), tr(painter))
        val bottom = beside(bl(painter), br(painter))
        below(bottom, top)
    }

/** The prose's first `flipped-pairs`, written out directly. */
public fun flippedPairs(painter: Painter): Painter {
    val painter2 = beside(painter, flipVert(painter))
    return below(painter2, painter2)
}

/** The prose's `flipped-pairs` as an instance of [squareOfFour]. */
public val flippedPairsViaSquareOfFour: (Painter) -> Painter =
    squareOfFour({ p -> p }, ::flipVert, { p -> p }, ::flipVert)

/** The prose's direct `square-limit`. */
public fun squareLimit(
    painter: Painter,
    n: Int,
): Painter {
    val quarter = cornerSplit(painter, n)
    val half = beside(flipHoriz(quarter), quarter)
    return below(flipVert(half), half)
}

/**
 * `flipHoriz` arrives with exercise 2.50, but the direct `squareLimit`
 * above needs it; this file carries the same transform-painter derivation
 * the exercise produces, and 2.50's solution proves its own equivalent.
 */
private fun flipHoriz(painter: Painter): Painter =
    transformPainter(
        painter,
        makeVect(1.0, 0.0),
        makeVect(0.0, 0.0),
        makeVect(1.0, 1.0),
    )

/** The prose's `square-limit` expressed with [squareOfFour]. */
public fun squareLimitViaSquareOfFour(
    painter: Painter,
    n: Int,
): Painter {
    val combine4 =
        squareOfFour(
            ::flipHoriz,
            { p -> p },
            ::rotate180Given,
            ::flipVert,
        )
    return combine4(cornerSplit(painter, n))
}

private fun rotate180Given(painter: Painter): Painter =
    transformPainter(
        painter,
        makeVect(1.0, 1.0),
        makeVect(0.0, 1.0),
        makeVect(1.0, 0.0),
    )

/** Renders a painter into a complete SVG document of the given size. */
public fun renderSvg(
    painter: Painter,
    width: Int = 300,
    height: Int = 300,
): String {
    val frame =
        Frame(
            makeVect(0.0, 0.0),
            makeVect(width.toDouble(), 0.0),
            makeVect(0.0, height.toDouble()),
        )
    val out = StringBuilder()
    painter(frame, out)
    // SVG's y axis grows downward while the book's frames grow upward, so
    // the drawing group is wrapped in a flip instead of changing the
    // painter algebra.
    return "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 $width $height\">" +
        "<g transform=\"scale(1,-1) translate(0,-$height)\">" +
        out +
        "</g></svg>"
}

/**
 * Captures the transformed segments a painter draws into a frame, for
 * geometric assertions: paints into a scratch buffer and parses the SVG
 * `<line>` elements back out, in draw order.
 */
public fun renderSegments(
    painter: Painter,
    frame: Frame,
): List<Segment> {
    val out = StringBuilder()
    painter(frame, out)
    return LINE_ELEMENT
        .findAll(out)
        .map { match ->
            val (x1, y1, x2, y2) = match.destructured
            makeSegment(makeVect(x1.toDouble(), y1.toDouble()), makeVect(x2.toDouble(), y2.toDouble()))
        }.toList()
}

private val LINE_ELEMENT =
    Regex("""<line x1="([-\d.]+)" y1="([-\d.]+)" x2="([-\d.]+)" y2="([-\d.]+)"/>""")
