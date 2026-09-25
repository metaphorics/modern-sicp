// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.9: the width algebra. The width of a sum (or difference) is
 * a function of the argument widths alone; multiplication and division
 * are demonstrated not to have that property.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.9 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** An interval holds the range [lo, hi] of an inexact quantity. */
export type Interval = { readonly lo: number; readonly hi: number };

/** Adds two intervals bound-wise. */
export function addInterval(_x: Interval, _y: Interval): Interval {
  throw new PendingSolution();
}

/** The width of an interval: half the difference of its bounds. */
export function width(_i: Interval): number {
  throw new PendingSolution();
}

/** True when the width of x + y equals width(x) + width(y) exactly. */
export function addWidthLaw(_x: Interval, _y: Interval): boolean {
  throw new PendingSolution();
}
