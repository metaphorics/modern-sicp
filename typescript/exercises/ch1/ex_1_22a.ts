// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.22a: a measurement discipline for the timed prime search.
 * Warmup round first, then N timed rounds, reporting the median of the
 * round times - never a single run.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.22a is not solved yet");
    this.name = "PendingSolution";
  }
}

export interface PrimeSearchTiming {
  readonly primes: number[];
  readonly medianMillis: number;
  readonly samples: number[];
}

/** Warmup, then N timed rounds of the search above `start`, median reported. */
export function searchForPrimesMedian(_start: number, _rounds?: number): PrimeSearchTiming {
  throw new PendingSolution();
}
