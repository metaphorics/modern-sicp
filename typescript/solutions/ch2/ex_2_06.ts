// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.6: Church numerals one, two, and churchAdd, defined
 * directly. one applies its step exactly once, two exactly twice, and
 * churchAdd composes the two numerals' step applications in sequence:
 * m(f) applied to n(f)(x) runs n's applications first and then m's. The
 * zero and add1 of the statement are kept for the substitution check.
 */
/** A Church numeral applies its step f, n times, to its seed x. */
export type Church = <T>(f: (x: T) => T) => (x: T) => T;

/** The statement's zero: the function that ignores its step. */
export const churchZero: Church = (_f) => (x) => x;

/** The statement's add-1: applies the step one more time than n does. */
export const add1 =
  (n: Church): Church =>
  (f) =>
  (x) =>
    f(n(f)(x));

/** One, defined directly: (add1 churchZero) with the substitution run. */
export const one: Church = (f) => (x) => f(x);

/** Two, defined directly. */
export const two: Church = (f) => (x) => f(f(x));

/** Addition, defined directly: m's composition after n's, not repeated add1. */
export const churchAdd =
  (m: Church, n: Church): Church =>
  (f) =>
  (x) =>
    m(f)(n(f)(x));

/** Decodes a numeral: applies it to the successor step and a zero seed. */
export const churchToInt = (n: Church): number => n((x: number) => x + 1)(0);
