// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.44: smooth returns the function whose value at x is the
 * average of f(x - dx), f(x), and f(x + dx); nFoldSmooth applies smooth n
 * times via repeated from exercise 1.43.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.44 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The smoothed version of f for the step dx. */
export function smooth(_f: (x: number) => number, _dx: number): (x: number) => number {
  throw new PendingSolution();
}

/** The n-fold smoothed version of f. */
export function nFoldSmooth(
  _f: (x: number) => number,
  _n: number,
  _dx: number,
): (x: number) => number {
  throw new PendingSolution();
}
