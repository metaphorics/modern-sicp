// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.17: multiplication by repeated addition, logarithmic in b,
 * in terms of double and halve.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.17 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Doubles an integer in terms of addition only. */
export function double(_x: number): number {
  throw new PendingSolution();
}

/** Divides an even integer by 2 in terms of halving only. */
export function halve(_x: number): number {
  throw new PendingSolution();
}

/** a * b by a logarithmic number of steps, assuming addition only. */
export function times(_a: number, _b: number): number {
  throw new PendingSolution();
}
