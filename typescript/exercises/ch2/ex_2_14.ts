// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.14: Lem is right. Algebraically equivalent formulas give
 * different answers; A/A is not the exact point [1, 1], and par1 and
 * par2 disagree on the same two resistors.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.14 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** An interval holds the range [lo, hi] of an inexact quantity. */
export type Interval = { readonly lo: number; readonly hi: number };

/** Adds two intervals bound-wise. */
export function addInterval(_x: Interval, _y: Interval): Interval {
  throw new PendingSolution();
}

/** Multiplies two intervals through the four corner products. */
export function mulInterval(_x: Interval, _y: Interval): Interval {
  throw new PendingSolution();
}

/** Divides x by the reciprocal of y, the section's pre-2.10 spelling. */
export function divInterval(_x: Interval, _y: Interval): Interval {
  throw new PendingSolution();
}

/** The midpoint of the two bounds. */
export function center(_i: Interval): number {
  throw new PendingSolution();
}

/** Half the difference of the two bounds. */
export function width(_i: Interval): number {
  throw new PendingSolution();
}

/** The percentage tolerance of an interval in center-percent form. */
export function percent(_i: Interval): number {
  throw new PendingSolution();
}

/** Lem's direct formula: (r1 * r2) / (r1 + r2). */
export function par1(_r1: Interval, _r2: Interval): Interval {
  throw new PendingSolution();
}

/** Lem's rewritten formula: 1 / (1/r1 + 1/r2). */
export function par2(_r1: Interval, _r2: Interval): Interval {
  throw new PendingSolution();
}
