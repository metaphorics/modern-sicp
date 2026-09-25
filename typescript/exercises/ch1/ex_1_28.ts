// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.28: the Miller-Rabin test that cannot be fooled. The
 * pending artifacts are the signaling expmod, the test itself, and the
 * known primes and non-primes it separates.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.28 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** expmod that returns 0 on discovering a nontrivial square root of 1. */
export function mrExpmod(_base: number, _exp: number, _m: number): number {
  throw new PendingSolution();
}

/** Miller-Rabin with an explicit witness, analogous to fermatTest. */
export function millerRabinWitness(_n: number, _a: number): boolean {
  throw new PendingSolution();
}
