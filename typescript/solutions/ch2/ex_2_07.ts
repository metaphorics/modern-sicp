// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.7: the interval selectors. The statement's constructor is
 * the literal pair of bounds; lowerBound and upperBound complete the
 * abstraction, and nothing outside the two selectors can see which
 * position is which.
 */
type Interval = { readonly lo: number; readonly hi: number };

/** The statement's constructor: the two bounds glued in order. */
export const makeInterval = (a: number, b: number): Interval => ({ lo: a, hi: b });

/** The lower bound of the interval. */
export const lowerBound = (x: Interval): number => x.lo;

/** The upper bound of the interval. */
export const upperBound = (x: Interval): number => x.hi;

/** Adds two intervals bound-wise, for the round-trip check. */
export const addInterval = (x: Interval, y: Interval): Interval => ({
  lo: lowerBound(x) + lowerBound(y),
  hi: upperBound(x) + upperBound(y),
});
