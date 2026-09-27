// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.7: the interval selectors. The statement postulates the
 * constructor makeInterval as the literal pair of bounds; lowerBound and
 * upperBound complete the abstraction.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.7 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** An interval holds the range [lo, hi] of an inexact quantity. */
export type Interval = { readonly lo: number; readonly hi: number };

/** The statement's constructor: the two bounds glued in order. */
export function makeInterval(_a: number, _b: number): Interval {
  throw new PendingSolution();
}

/** The lower bound of the interval. */
export function lowerBound(_x: Interval): number {
  throw new PendingSolution();
}

/** The upper bound of the interval. */
export function upperBound(_x: Interval): number {
  throw new PendingSolution();
}
