// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 3.8: a procedure exposing the host's operand evaluation
 * order. Pending scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_08.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.8 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Builds the probe procedure f used in `(f 0) + (f 1)`. */
export function makeOrderProbe(): (x: number) => number {
  throw new PendingSolution();
}
