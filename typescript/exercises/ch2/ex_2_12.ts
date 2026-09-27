// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.12: the center-percent constructor and the percent selector.
 * The center selector is the main text's, unchanged.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.12 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** An interval holds the range [lo, hi] of an inexact quantity. */
export type Interval = { readonly lo: number; readonly hi: number };

/** Builds the interval centered at c with percentage tolerance p. */
export function makeCenterPercent(_c: number, _p: number): Interval {
  throw new PendingSolution();
}

/** The midpoint of the two bounds: the main text's center. */
export function center(_i: Interval): number {
  throw new PendingSolution();
}

/** The percentage tolerance of the interval. */
export function percent(_i: Interval): number {
  throw new PendingSolution();
}
