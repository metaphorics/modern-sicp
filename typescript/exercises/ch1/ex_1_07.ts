// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.7: the absolute-tolerance good-enough? test fails for very
 * small and very large numbers. Design a square-root procedure whose end
 * test watches how the guess changes from one iteration to the next and
 * stops when the change is a very small fraction of the guess.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.7 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function sqrtRelative(_x: number): number {
  throw new PendingSolution();
}
