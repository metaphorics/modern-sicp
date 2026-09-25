// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.43: repeated(f, n) returns the function that applies f n
 * times; the statement suggests compose from exercise 1.42.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.43 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The n-th repeated application of f. */
export function repeated(_f: (x: number) => number, _n: number): (x: number) => number {
  throw new PendingSolution();
}
