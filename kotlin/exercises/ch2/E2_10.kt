// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.10

package sicp.ch2.exercises

import arrow.core.raise.Raise
import sicp.runtime.PendingSolution

// Interval, makeInterval, lowerBound, upperBound, and mulInterval are exercise 2.7's public declarations, reused here.

/** The domain's one error: dividing by an interval that spans zero. */
public sealed interface IntervalError {
    public data class SpansZero(
        val interval: Interval,
    ) : IntervalError
}

/**
 * Exercise 2.10: Ben Bitdiddle, an expert systems programmer, looks over
 * Alyssa's shoulder and comments that it is not clear what it means to
 * divide by an interval that spans zero. Modify `divInterval` (given as
 * plain `divInterval` in exercise 2.7) to check for this condition and to
 * raise `IntervalError.SpansZero` if it occurs, as `divIntervalChecked`.
 * The statement lives in the section 2.1 chapter text.
 *
 * The scaffold divides `[4, 8]` by `[1, 2]`, which does not span zero.
 */
context(r: Raise<IntervalError>)
public fun divIntervalChecked(
    x: Interval,
    y: Interval,
): Interval = throw PendingSolution()
