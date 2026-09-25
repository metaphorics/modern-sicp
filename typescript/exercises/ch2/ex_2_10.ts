// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.10: division checked against a divisor that spans zero.
 * The reciprocal of such an interval is not an interval, so the checked
 * procedure reports the condition as a typed error instead of a number.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.10 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** An interval holds the range [lo, hi] of an inexact quantity. */
export type Interval = { readonly lo: number; readonly hi: number };

/** The failure of a division whose divisor spans zero. */
export type DivError = { readonly _tag: "ZeroSpan"; readonly lo: number; readonly hi: number };

/** The book's two-sided result: a value or an error, never a throw. */
export type Result<A, E> =
  | { readonly _tag: "Ok"; readonly value: A }
  | { readonly _tag: "Error"; readonly error: E };

/** Divides x by y, reporting a ZeroSpan error when y spans or touches zero. */
export function divIntervalChecked(_x: Interval, _y: Interval): Result<Interval, DivError> {
  throw new PendingSolution();
}
