// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.4: the conditional picks which procedure to apply, then the
 * surrounding call applies it to a and b.
 *
 * TypeScript's `+` and `-` are syntax, not values, so the conditional
 * cannot return an operator name in this eager host; it returns a
 * procedure instead. For positive b the addition is applied; otherwise
 * the subtraction, and a - b = a + |b| exactly when b <= 0.
 */
export function aPlusAbsB(a: number, b: number): number {
  return (
    b > 0 ? (x: number, y: number): number => x + y : (x: number, y: number): number => x - y
  )(a, b);
}
