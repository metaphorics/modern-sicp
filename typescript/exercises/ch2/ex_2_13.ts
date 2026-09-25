// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.13: under small percentage tolerances, the percentage
 * tolerance of a product is approximately the sum of the factors'.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.13 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** An interval holds the range [lo, hi] of an inexact quantity. */
export type Interval = { readonly lo: number; readonly hi: number };

/** Builds the interval centered at c with percentage tolerance p. */
export function makeCenterPercent(_c: number, _p: number): Interval {
  throw new PendingSolution();
}

/** Multiplies two intervals through the four corner products. */
export function mulInterval(_x: Interval, _y: Interval): Interval {
  throw new PendingSolution();
}

/** The percentage tolerance of an interval. */
export function percent(_i: Interval): number {
  throw new PendingSolution();
}

/** The percentage tolerance of the product p * q. */
export function productPercent(_p: Interval, _q: Interval): number {
  throw new PendingSolution();
}
