// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.10

package sicp.ch2.exercises

import arrow.core.raise.Raise

// Interval, makeInterval, lowerBound, upperBound, and mulInterval are exercise 2.7's public declarations.

/** The domain's one error: dividing by an interval that spans zero. */
public sealed interface IntervalError {
    public data class SpansZero(
        val interval: Interval,
    ) : IntervalError
}

/** Same shape as `divInterval`, but raises instead of computing a meaningless reciprocal. */
context(r: Raise<IntervalError>)
public fun divIntervalChecked(
    x: Interval,
    y: Interval,
): Interval {
    if (y.lowerBound <= 0.0 && y.upperBound >= 0.0) r.raise(IntervalError.SpansZero(y))
    return mulInterval(x, makeInterval(1.0 / y.upperBound, 1.0 / y.lowerBound))
}
