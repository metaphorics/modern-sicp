// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.25: Alyssa P. Hacker's expmod simplification, and why it
 * does not serve the fast prime tester on this host. The pending
 * artifacts are her version, a bigint restatement, and the comparisons.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.25 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Alyssa's expmod: the whole exponential first, remainder after. */
export function expmodSimplified(_base: number, _exp: number, _m: number): number {
  throw new PendingSolution();
}

/** fast-expt over bigint, exact at any exponent. */
export function fastExptBigint(_base: bigint, _exp: number): bigint {
  throw new PendingSolution();
}

/** Alyssa's idea restated over bigint, exact but slower. */
export function expmodSimplifiedBigint(_base: bigint, _exp: number, _m: bigint): bigint {
  throw new PendingSolution();
}
