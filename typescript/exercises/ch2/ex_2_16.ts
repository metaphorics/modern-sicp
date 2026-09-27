// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.16: why equivalent algebraic expressions give different
 * answers, and whether an interval package without this shortcoming can
 * exist. The smallest instance is x - x, which the package cannot
 * cancel.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.16 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** An interval holds the range [lo, hi] of an inexact quantity. */
export type Interval = { readonly lo: number; readonly hi: number };

/** Subtracts y from x bound-wise, treating the two as independent. */
export function subInterval(_x: Interval, _y: Interval): Interval {
  throw new PendingSolution();
}

/** The package's answer for x - x on one interval: not the exact [0, 0]. */
export function xMinusX(_x: Interval): Interval {
  throw new PendingSolution();
}
