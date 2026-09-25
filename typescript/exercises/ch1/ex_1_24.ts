// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.24: the Fermat test in place of the divisor search,
 * timed with the warmup-plus-median discipline. Witnesses come from the
 * section's seeded Random. The pending artifact is the timed Fermat
 * search.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.24 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The three smallest primes above `start`, found by the Fermat test. */
export function searchForPrimesFermat(_start: number, _witnessRounds: number): number[] {
  throw new PendingSolution();
}

/** Median search time of the Fermat search, warmup discarded. */
export function measureFermatMedian(_start: number): { primes: number[]; medianMillis: number } {
  throw new PendingSolution();
}
