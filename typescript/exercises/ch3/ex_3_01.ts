// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 3.1: make-accumulator, a factory whose accumulators each own
 * a local sum. Pending scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_01.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.1 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Builds an accumulator answering the sum so far after each call. */
export function makeAccumulator(_initial: number): never {
  throw new PendingSolution();
}
