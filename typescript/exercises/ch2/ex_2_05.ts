// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.5: pairs of nonnegative integers encoded as 2^a * 3^b on
 * this edition's exact integers. The encoding outgrows the 53 exact bits
 * of number almost immediately, so the value stays in bigint.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.5 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Encodes the pair (a, b) as the integer 2^a * 3^b. */
export function cons(_a: bigint, _b: bigint): bigint {
  throw new PendingSolution();
}

/** Recovers a by counting how many times 2 divides the encoding. */
export function car(_p: bigint): bigint {
  throw new PendingSolution();
}

/** Recovers b by counting how many times 3 divides the encoding. */
export function cdr(_p: bigint): bigint {
  throw new PendingSolution();
}
