// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.38: Euler's continued fraction for e - 2 has all numerators 1
 * and denominators 1, 2, 1, 1, 4, 1, 1, 6, 1, 1, 8, ...; e is 2 plus the
 * fraction, computed with contFrac from exercise 1.37.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.38 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** e approximated by 2 + the k-term Euler fraction. */
export function eApprox(_k: number): number {
  throw new PendingSolution();
}
