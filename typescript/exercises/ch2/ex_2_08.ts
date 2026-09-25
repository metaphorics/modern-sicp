// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.8: the difference of two intervals, subtracting bound-wise
 * so the result covers every possible difference.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.8 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** An interval holds the range [lo, hi] of an inexact quantity. */
export type Interval = { readonly lo: number; readonly hi: number };

/** The interval x - y, covering every difference of a value of x and one of y. */
export function subInterval(_x: Interval, _y: Interval): Interval {
  throw new PendingSolution();
}
