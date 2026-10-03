// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.3: implement a function that takes three numbers as arguments
 * and returns the sum of the squares of the two larger numbers.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.3 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function sumOfSquaresOfTwoLarger(_a: number, _b: number, _c: number): number {
  throw new PendingSolution();
}
