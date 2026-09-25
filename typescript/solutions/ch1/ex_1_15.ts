// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.15: sine by argument reduction.
 *
 * p is applied once per division of the angle by 3, until the angle is
 * at most 0.1 radians, so the count is the number of divisions: about
 * log base 3 of a, in both space and steps.
 */
export const cube = (x: number): number => x * x * x;

export const p = (x: number): number => 3 * x - 4 * cube(x);

export const sine = (angle: number): number =>
  !(Math.abs(angle) > 0.1) ? angle : p(sine(angle / 3));

/** The counting variant: one p application per division by 3. */
export function sineWithCount(angle: number): { value: number; pApplications: number } {
  const rec = (a: number, count: number): { value: number; pApplications: number } => {
    if (!(Math.abs(a) > 0.1)) {
      return { value: a, pApplications: count };
    }
    const inner = rec(a / 3, count + 1);
    return { value: p(inner.value), pApplications: inner.pApplications };
  };
  return rec(angle, 0);
}
