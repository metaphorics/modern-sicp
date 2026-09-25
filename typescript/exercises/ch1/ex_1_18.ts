// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.18: the Russian peasant method: an iterative process for
 * multiplying two integers in terms of adding, doubling, and halving.
 * The pending artifact is the loop and its state trace.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.18 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** a * b as an explicit loop over (x, y, acc) with x * y + acc invariant. */
export function timesIter(_a: number, _b: number): number {
  throw new PendingSolution();
}

/** The sequence of (acc, x, y) states, to watch the invariant hold. */
export function timesIterStates(
  _a: number,
  _b: number,
): Array<{ acc: number; x: number; y: number }> {
  throw new PendingSolution();
}
