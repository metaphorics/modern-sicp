// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.8: the difference of two intervals. The most negative
 * possible difference is lower(x) - upper(y), the most positive is
 * upper(x) - lower(y); the bounds land there no matter where zero sits,
 * which is the same reasoning Alyssa used for the sum, with the second
 * interval's bounds crossed.
 */
type Interval = { readonly lo: number; readonly hi: number };

/** The interval x - y, covering every difference of a value of x and one of y. */
export const subInterval = (x: Interval, y: Interval): Interval => ({
  lo: x.lo - y.hi,
  hi: x.hi - y.lo,
});

/** The width of an interval: half the difference of its bounds. */
export const width = (i: Interval): number => (i.hi - i.lo) / 2;
