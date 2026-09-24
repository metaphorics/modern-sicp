// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.45

package sicp.ch2.exercises

/**
 * The general splitting operation: [bigOp] combines the painter with a
 * pair of smaller copies combined by [smallOp]. `rightSplit` is
 * `split(::beside, ::below)`; `upSplit` is `split(::below, ::beside)`.
 */
public fun split(
    bigOp: (Painter, Painter) -> Painter,
    smallOp: (Painter, Painter) -> Painter,
): (Painter, Int) -> Painter {
    fun go(
        painter: Painter,
        n: Int,
    ): Painter =
        if (n == 0) {
            painter
        } else {
            val smaller = go(painter, n - 1)
            bigOp(painter, smallOp(smaller, smaller))
        }
    return ::go
}

/** `rightSplit` re-derived from [split]. */
public val rightSplitViaSplit: (Painter, Int) -> Painter = split(::beside, ::below)

/** `upSplit` re-derived from [split]. */
public val upSplitViaSplit: (Painter, Int) -> Painter = split(::below, ::beside)

/** Renders `split(::beside, ::below)(wave, 1)` and counts its segments. */
public fun ex_2_45(): Int = renderSegments(rightSplitViaSplit(wave, 1), unitSquare).size
