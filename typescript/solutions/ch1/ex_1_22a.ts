// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.22a: warmup plus median timings for the prime search.
 *
 * A raw first-run timing lies on a just-in-time machine: the first
 * passes compile and optimize the hot functions, the clock is coarsened
 * (about 100 microseconds per tick without cross-origin isolation), and
 * a single sample is one scheduler hiccup from useless. The harness
 * runs the whole search once as a warmup whose times are discarded,
 * then takes `rounds` timed samples and reports their median. The
 * workload starts above 100,000,000 so one search runs long enough to
 * survive the clock's coarsening.
 */
export const divides = (a: number, b: number): boolean => b % a === 0;

export const findDivisor = (n: number, testDivisor: number): number => {
  let d = testDivisor;
  while (d * d <= n) {
    if (divides(d, n)) {
      return d;
    }
    d += 1;
  }
  return n;
};

export const smallestDivisor = (n: number): number => findDivisor(n, 2);

export const isPrime = (n: number): boolean => n === smallestDivisor(n);

export const searchForPrimes = (start: number): number[] => {
  const found: number[] = [];
  let candidate = start % 2 === 0 ? start + 1 : start;
  while (found.length < 3) {
    if (isPrime(candidate)) {
      found.push(candidate);
    }
    candidate += 2;
  }
  return found;
};

export interface PrimeSearchTiming {
  readonly primes: number[];
  readonly medianMillis: number;
  readonly samples: number[];
}

const timedSearch = (start: number): number => {
  const began = performance.now();
  searchForPrimes(start);
  return performance.now() - began;
};

const medianOf = (values: number[]): number => {
  const sorted = [...values].sort((x, y) => x - y);
  const middle = Math.floor(sorted.length / 2);
  const upper = sorted[middle];
  if (upper === undefined) {
    throw new RangeError("median of an empty sample");
  }
  if (sorted.length % 2 === 0) {
    const lower = sorted[middle - 1];
    return lower === undefined ? upper : (lower + upper) / 2;
  }
  return upper;
};

export function searchForPrimesMedian(start: number, rounds = 9): PrimeSearchTiming {
  const primes = searchForPrimes(start);
  searchForPrimes(start);
  const samples: number[] = [];
  for (let round = 0; round < rounds; round += 1) {
    samples.push(timedSearch(start));
  }
  return { primes, medianMillis: medianOf(samples), samples };
}
