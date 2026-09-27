// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.29: Simpson's Rule integration of f over [a, b] with n even
 * panels, h = (b - a)/n and weights 1, 4, 2, ..., 2, 4, 1.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.29 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The Simpson's Rule approximation of the integral of f from a to b. */
export function simpson(_f: (x: number) => number, _a: number, _b: number, _n: number): number {
  throw new PendingSolution();
}
