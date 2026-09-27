// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.44: smoothing as a function transformer.
 *
 * smooth(f, dx) is the function whose value at x is the average of
 * f(x - dx), f(x), and f(x + dx); nFoldSmooth applies that one-step
 * transformation n times, the book's ((repeated smooth n) f) with the
 * repetition folded over f itself, so n = 0 returns f unchanged.
 * Smoothing matters most near a maximum or minimum, where the
 * derivative vanishes but measurement noise does not.
 */
export function smooth(f: (x: number) => number, dx: number): (x: number) => number {
  return (x: number): number => (f(x - dx) + f(x) + f(x + dx)) / 3;
}

/** The n-fold smoothed version of f. */
export function nFoldSmooth(
  f: (x: number) => number,
  n: number,
  dx: number,
): (x: number) => number {
  let g = f;
  for (let i = 0; i < n; i++) {
    g = smooth(g, dx);
  }
  return g;
}
