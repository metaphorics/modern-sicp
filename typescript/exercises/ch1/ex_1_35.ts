// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.35: the golden ratio is the fixed point of x |-> 1 + 1/x;
 * compute it with the section's fixedPoint search.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.35 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** phi via fixedPoint((x) => 1 + 1/x, 1.0). */
export function goldenRatio(): number {
  throw new PendingSolution();
}
