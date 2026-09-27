// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.33: filteredAccumulate combines only the terms whose range
 * value satisfies the predicate. Artifacts: the abstraction, the sum of
 * squares of primes in a..b, and the product of integers below n that are
 * relatively prime to n.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.33 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** accumulate restricted to the values of the range that pass filter. */
export function filteredAccumulate(
  _combiner: (x: number, y: number) => number,
  _nullValue: number,
  _term: (x: number) => number,
  _a: number,
  _next: (x: number) => number,
  _b: number,
  _filter: (x: number) => boolean,
): number {
  throw new PendingSolution();
}

/** The sum of the squares of the primes in the interval a..b. */
export function sumOfSquaresOfPrimes(_a: number, _b: number): number {
  throw new PendingSolution();
}

/** The product of the positive integers i < n with gcd(i, n) = 1. */
export function productOfRelativelyPrimes(_n: number): number {
  throw new PendingSolution();
}
