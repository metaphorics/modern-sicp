// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.34

package sicp.ch2.exercises

/**
 * Horner's rule, as an accumulation: initialize the result to 0, and at
 * each coefficient add it and multiply by x. The book's convention lists
 * coefficients from `a0` through `an`, lowest degree first, matching
 * `accumulateList`'s right-to-left fold.
 */
public fun hornerEval(
    x: Long,
    coefficientSequence: List<Long>,
): Long = accumulateList({ coeff, higher -> coeff + x * higher }, 0L, coefficientSequence)

/** `hornerEval(2, listOf(1, 3, 0, 5, 0, 1))` evaluates `1 + 3x + 5x^3 + x^5` at x = 2. */
public fun ex_2_34(): Long = hornerEval(2L, listOf(1L, 3L, 0L, 5L, 0L, 1L))
