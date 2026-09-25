// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.8: Newton's method for cube roots: if y approximates the
 * cube root of x, then (x / y^2 + 2y) / 3 is a better approximation.
 * Implement a cube-root procedure analogous to the square-root procedure.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.8 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function cubeRoot(_x: number): number {
  throw new PendingSolution();
}
