// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.20: count every remainder the eager gcd computes while
 * reducing gcd(206, 40). The pending artifact is the instrumented gcd.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.20 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The eager gcd, reporting its value and its remainder count. */
export function tracedGcd(_a: number, _b: number): { value: number; remainderCalls: number } {
  throw new PendingSolution();
}
