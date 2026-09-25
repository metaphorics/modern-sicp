// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.15: Eva Lu Ator's claim. A formula that never repeats an
 * uncertain variable produces tighter bounds, so par2 beats par1 on the
 * same resistors.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.15 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** An interval holds the range [lo, hi] of an inexact quantity. */
export type Interval = { readonly lo: number; readonly hi: number };

/** True when par2's percentage tolerance is the tighter of the two. */
export function par2IsTighter(_par1: Interval, _par2: Interval): boolean {
  throw new PendingSolution();
}
