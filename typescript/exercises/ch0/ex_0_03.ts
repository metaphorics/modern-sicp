// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 0.3: the recursion floor of the machine.
 *
 * `sumToRecursive` takes a frame per call and refuses deep enough inputs
 * with a `RangeError`; `sumToIterative` runs the same process as a loop at
 * a constant stack; `recursionDepth` reports how deep plain recursion
 * reaches before the engine refuses. The statement lives in the section
 * 0.3 chapter text.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 0.3 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function sumToRecursive(_n: number): number {
  throw new PendingSolution();
}

export function sumToIterative(_n: number): number {
  throw new PendingSolution();
}

export function recursionDepth(): number {
  throw new PendingSolution();
}
