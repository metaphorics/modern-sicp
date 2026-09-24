// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.47

package sicp.ch2.exercises

/**
 * The book's first constructor, `(list origin edge1 edge2)`; this
 * package's `Frame` data class (declared in `Painters.kt`, carried early
 * for the same reason `Vect` is) already plays that role, with
 * `originFrame`/`edge1Frame`/`edge2Frame` as its selectors.
 *
 * The book's second constructor, `(cons origin (cons edge1 edge2))`, is
 * this alternative representation: a pair of the origin and a pair of
 * the two edges.
 */
public typealias FramePair = Pair<Vect, Pair<Vect, Vect>>

public fun makeFramePair(
    origin: Vect,
    edge1: Vect,
    edge2: Vect,
): FramePair = origin to (edge1 to edge2)

public fun originFramePair(frame: FramePair): Vect = frame.first

public fun edge1FramePair(frame: FramePair): Vect = frame.second.first

public fun edge2FramePair(frame: FramePair): Vect = frame.second.second

/** [frameCoordMap], rebuilt over the pair representation's selectors. */
public fun frameCoordMapPair(frame: FramePair): (Vect) -> Vect =
    { v ->
        addVect(
            originFramePair(frame),
            addVect(
                scaleVect(edge1FramePair(frame), xcorVect(v)),
                scaleVect(edge2FramePair(frame), ycorVect(v)),
            ),
        )
    }

/** True when both constructors' coordinate maps agree on the unit square's corners. */
public fun ex_2_47(): Boolean {
    val origin = makeVect(1.0, 1.0)
    val edge1 = makeVect(2.0, 0.0)
    val edge2 = makeVect(0.0, 3.0)
    val listFrame = Frame(origin, edge1, edge2)
    val pairFrame = makeFramePair(origin, edge1, edge2)
    val corners = listOf(makeVect(0.0, 0.0), makeVect(1.0, 0.0), makeVect(0.0, 1.0), makeVect(1.0, 1.0))
    return corners.all { frameCoordMap(listFrame)(it) == frameCoordMapPair(pairFrame)(it) }
}
