// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.23: skip even divisors with a next procedure, and
 * remeasure. The pending artifacts are next, the modified divisor
 * search, and the measured ratio of the two searches.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.23 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Returns 3 for input 2, otherwise the input plus 2. */
export function next(_n: number): number {
  throw new PendingSolution();
}

/** The smallest divisor greater than 1, stepping with next. */
export function smallestDivisorNext(_n: number): number {
  throw new PendingSolution();
}

/** The median search time with the plain and the skipping divisor loop. */
export function measureRatio(_start: number): {
  plainMillis: number;
  skippingMillis: number;
  ratio: number;
} {
  throw new PendingSolution();
}
