// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.39: Lambert's tangent continued fraction.
 *
 * tan x = x/(1 - x^2/(3 - x^2/(5 - ...))) with x in radians: the first
 * numerator is x, all later numerators are x^2, and the denominators are
 * the odd numbers 1, 3, 5, .... The fraction is folded up from term k with
 * subtractions in place of the additions of exercise 1.37.
 */
export function tanCf(x: number, k: number): number {
  let acc = 0;
  for (let i = k; i >= 1; i -= 1) {
    const numerator = i === 1 ? x : x * x;
    acc = numerator / (2 * i - 1 - acc);
  }
  return acc;
}
