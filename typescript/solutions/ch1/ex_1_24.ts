// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.24: the Fermat test, timed honestly.
 *
 * timedPrimeTest from exercise 1.22 wraps fastPrime instead of isPrime;
 * witnesses come from the section's seeded Random, the stand-in for
 * the original's random primitive (Math.random would have the same shape but
 * no replay). The medians use the warmup-plus-median discipline of
 * exercise 1.22a, because a first-run timing on a JIT measures the
 * compiler, not the test.
 */
import type { Random } from "../../examples/ch1/random.js";

export const expmod = (base: number, exp: number, m: number): number => {
  if (exp === 0) {
    return 1;
  }
  if (exp % 2 === 0) {
    const squared = (inner: number): number => inner * inner;
    return squared(expmod(base, exp / 2, m)) % m;
  }
  return (base * expmod(base, exp - 1, m)) % m;
};

export const fermatTest = (n: number, rng: Random): boolean => {
  const a = 1 + rng.random(n - 1);
  return expmod(a, n, n) === a;
};

export const fastPrime = (n: number, times: number, rng: Random): boolean => {
  let remaining = times;
  while (remaining > 0) {
    if (!fermatTest(n, rng)) {
      return false;
    }
    remaining -= 1;
  }
  return true;
};

export const searchForPrimesFermat = (
  start: number,
  witnessRounds: number,
  rng: Random,
): number[] => {
  const found: number[] = [];
  let candidate = start % 2 === 0 ? start + 1 : start;
  while (found.length < 3) {
    if (fastPrime(candidate, witnessRounds, rng)) {
      found.push(candidate);
    }
    candidate += 2;
  }
  return found;
};

export function measureFermatMedian(
  start: number,
  witnessRounds: number,
  rounds: number,
  rng: Random,
): { primes: number[]; medianMillis: number } {
  const primes = searchForPrimesFermat(start, witnessRounds, rng);
  const timed = (): number => {
    const began = performance.now();
    searchForPrimesFermat(start, witnessRounds, rng);
    return performance.now() - began;
  };
  const samples: number[] = [];
  for (let round = 0; round < rounds; round += 1) {
    samples.push(timed());
  }
  const sorted = [...samples].sort((x, y) => x - y);
  const median = sorted[Math.floor(sorted.length / 2)];
  if (median === undefined) {
    throw new RangeError("median of an empty sample");
  }
  return { primes, medianMillis: median };
}
