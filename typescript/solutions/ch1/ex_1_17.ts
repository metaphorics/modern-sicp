// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.17: multiplication in terms of addition, double, and halve.
 *
 * The step rule mirrors fast-expt: halving an even b squares the
 * addend's weight by doubling a; an odd b peels one copy of a off and
 * recurses on b - 1. The recursion depth is the step count, about
 * log2(b) halvings plus the pops of odd remainders.
 */
export const double = (x: number): number => x + x;

export const halve = (x: number): number => x / 2;

export const times = (a: number, b: number): number => {
  if (b === 0) {
    return 0;
  }
  if (b % 2 === 0) {
    return times(double(a), halve(b));
  }
  return a + times(a, b - 1);
};
