// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.11: multiplication by nine sign-tested cases, only one of
 * which needs all four corner products.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.11 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** An interval holds the range [lo, hi] of an inexact quantity. */
export type Interval = { readonly lo: number; readonly hi: number };

/** The sign class of an interval: positive, negative, or spanning zero. */
export type Sign = "positive" | "negative" | "span";

/** Classifies an interval by the signs of its endpoints. */
export function signOf(_x: Interval): Sign {
  throw new PendingSolution();
}

/** Multiplies through the one of nine cases the two sign classes select. */
export function mulIntervalFast(_x: Interval, _y: Interval): Interval {
  throw new PendingSolution();
}
