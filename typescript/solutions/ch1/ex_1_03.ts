// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.3: the sum of the squares of the two larger of three
 * numbers.
 *
 * The sum of all three squares minus the square of the smallest: the
 * two larger numbers contribute exactly once, whichever positions they
 * sit in, so no case analysis on the ordering is needed.
 */
export function sumOfSquaresOfTwoLarger(a: number, b: number, c: number): number {
  const smallest = Math.min(a, b, c);
  return a * a + b * b + c * c - smallest * smallest;
}
