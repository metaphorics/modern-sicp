// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.22: timed prime search with performance.now(). The
 * timedPrimeTest of the statement is given; the pending artifact is the
 * search over consecutive odd integers.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.22 is not solved yet");
    this.name = "PendingSolution";
  }
}

export interface TimedPrime {
  readonly n: number;
  readonly elapsedMillis: number;
}

/** The statement's timed prime test: null when n is not prime. */
export const timedPrimeTest = (_n: number): TimedPrime | null => {
  throw new PendingSolution();
};

/** The three smallest primes larger than `start`. */
export function searchForPrimes(_start: number): number[] {
  throw new PendingSolution();
}
