// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.1: a better makeRat that handles both positive and negative
 * arguments. The sign is normalized so that a negative rational carries it
 * on the numerator alone.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.1 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Builds the reduced rational n/d with the sign on the numerator alone. */
export function makeRatNormalized(_n: bigint, _d: bigint): readonly [bigint, bigint] {
  throw new PendingSolution();
}
