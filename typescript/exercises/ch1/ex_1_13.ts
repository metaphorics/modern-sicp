// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.13: prove that Fib(n) is the closest integer to
 * phi^n / sqrt(5), where phi = (1 + sqrt(5)) / 2. The pending artifact
 * is the computational cross-check the proof leans on: the closed form
 * as a function, compared against the direct definition.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.13 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The closed form (phi^n - psi^n) / sqrt(5) as a number. */
export function closedFormFib(_n: number): number {
  throw new PendingSolution();
}
