// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.22: a timed search for primes over odd integers.
 *
 * timedPrimeTest is the statement's shape: performance.now() before and
 * after the primality test, milliseconds as a float. searchForPrimes
 * walks the odd integers above `start` until it holds three primes.
 * The book's three-asterisk print becomes the returned record, since a
 * test reads data, not a display line. The clock is coarsened (about
 * 100 microseconds per tick without cross-origin isolation) and the JIT
 * warms up during the search, so the small elapsed values are orders of
 * magnitude, not measurements; exercise 1.22a builds the honest
 * harness.
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

export interface TimedPrime {
  readonly n: number;
  readonly elapsedMillis: number;
}

export const timedPrimeTest = (n: number): TimedPrime | null => {
  const start = performance.now();
  return isPrime(n) ? { n, elapsedMillis: performance.now() - start } : null;
};

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
