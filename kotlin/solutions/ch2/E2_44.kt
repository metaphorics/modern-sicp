// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.44

package sicp.ch2.exercises

/**
 * `up-split`, switching the roles of [below] and [beside] relative to
 * `rightSplit`: it branches upward instead of to the right.
 */
public fun upSplit(
    painter: Painter,
    n: Int,
): Painter =
    if (n == 0) {
        painter
    } else {
        val smaller = upSplit(painter, n - 1)
        below(painter, beside(smaller, smaller))
    }

/** Renders `upSplit(wave, 1)` into [unitSquare] and counts its segments. */
public fun ex_2_44(): Int = renderSegments(upSplit(wave, 1), unitSquare).size
