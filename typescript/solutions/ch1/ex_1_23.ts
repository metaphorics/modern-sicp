// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.23: skipping even divisors, remeasured.
 *
 * next steps 2 -> 3 -> 5 -> 7 -> ... so the trial divisions halve. The
 * remeasurement uses the warmup-plus-median discipline of exercise
 * 1.22a at the same workload (the three primes above 100,000,000),
 * because a raw single run measures the JIT and the clock's coarsening
 * more than the algorithm. The measured ratio answers the statement's
 * question: not 2, but somewhat less, for the reasons in the rationale.
 */
export const divides = (a: number, b: number): boolean => b % a === 0;

export const next = (n: number): number => (n === 2 ? 3 : n + 2);

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

export const findDivisorNext = (n: number, testDivisor: number): number => {
  let d = testDivisor;
  while (d * d <= n) {
    if (divides(d, n)) {
      return d;
    }
    d = next(d);
  }
  return n;
};

export const smallestDivisor = (n: number): number => findDivisor(n, 2);

export const smallestDivisorNext = (n: number): number => findDivisorNext(n, 2);

export const isPrimePlain = (n: number): boolean => n === smallestDivisor(n);

export const isPrimeNext = (n: number): boolean => n === smallestDivisorNext(n);

const searchWith = (start: number, prime: (n: number) => boolean): number[] => {
  const found: number[] = [];
  let candidate = start % 2 === 0 ? start + 1 : start;
  while (found.length < 3) {
    if (prime(candidate)) {
      found.push(candidate);
    }
    candidate += 2;
  }
  return found;
};

const medianSearchTime = (start: number, prime: (n: number) => boolean, rounds: number): number => {
  const timed = (): number => {
    const began = performance.now();
    searchWith(start, prime);
    return performance.now() - began;
  };
  searchWith(start, prime);
  const samples: number[] = [];
  for (let round = 0; round < rounds; round += 1) {
    samples.push(timed());
  }
  const sorted = [...samples].sort((x, y) => x - y);
  const median = sorted[Math.floor(sorted.length / 2)];
  if (median === undefined) {
    throw new RangeError("median of an empty sample");
  }
  return median;
};

export function measureRatio(start: number): {
  plainMillis: number;
  skippingMillis: number;
  ratio: number;
} {
  const rounds = 9;
  const plainMillis = medianSearchTime(start, isPrimePlain, rounds);
  const skippingMillis = medianSearchTime(start, isPrimeNext, rounds);
  return { plainMillis, skippingMillis, ratio: plainMillis / skippingMillis };
}
