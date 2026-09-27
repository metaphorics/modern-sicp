// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.12: the center-percent constructor and the percent selector.
 * makeCenterPercent converts the percentage into a half-width around the
 * center's magnitude; percent inverts it. The center selector is the main
 * text's, unchanged.
 */
type Interval = { readonly lo: number; readonly hi: number };

/** Builds the interval centered at c with percentage tolerance p. */
export const makeCenterPercent = (c: number, p: number): Interval => {
  const half = (Math.abs(c) * p) / 100;
  return { lo: c - half, hi: c + half };
};

/** The midpoint of the two bounds: the main text's center. */
export const center = (i: Interval): number => (i.lo + i.hi) / 2;

/** Half the difference of the two bounds: the main text's width. */
export const width = (i: Interval): number => (i.hi - i.lo) / 2;

/** The percentage tolerance of the interval. */
export const percent = (i: Interval): number => (100 * width(i)) / Math.abs(center(i));
