// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.5: Ben Bitdiddle's evaluation-order test, answered
 * empirically.
 *
 * The host is applicative-order: `test(0, p())` evaluates the argument
 * expression `p()` before the call, and that evaluation diverges; the
 * observable end is the host's stack-overflow RangeError, not 0. The
 * `testThunk` stand-in evaluates the alternative lazily and shows what
 * normal order would do: return 0 without ever calling its argument.
 */
export const p = (): never => p();

export const test = (x: number, y: number): number => (x === 0 ? 0 : y);

/** The normal-order stand-in: y is a thunk, run only when x is not 0. */
export const testThunk = (x: number, y: () => number): number => (x === 0 ? 0 : y());

export function ex_1_05(): string {
  return (
    "Applicative-order evaluation (the host's rule) evaluates the argument p() " +
    "first, diverges, and never returns 0; normal-order evaluation would " +
    "substitute the operand expression for y and return 0 without calling p."
  );
}
