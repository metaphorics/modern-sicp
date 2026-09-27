// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.30: the summation of sum as a while loop whose state is the
 * running result and the current value of a (Node gives no tail-call
 * guarantee, so the iterative shape is a loop).
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.30 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The loop-shaped sum: same values as the section's recursive sum. */
export function sumIter(
  _term: (x: number) => number,
  _a: number,
  _next: (x: number) => number,
  _b: number,
): number {
  throw new PendingSolution();
}
