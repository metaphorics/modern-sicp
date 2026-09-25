// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 3.1a (added by this edition, extends exercise 3.1): the
 * accumulator that also answers the transaction history behind each
 * sum. Pending scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_01a.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.1a is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Builds an accumulator answering sum plus history after each call. */
export function makeAccumulatorWithHistory(_initial: number): never {
  throw new PendingSolution();
}
