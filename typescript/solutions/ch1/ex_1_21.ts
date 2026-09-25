// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.21: smallest divisors of 199, 1999, and 19999.
 *
 * The section's smallestDivisor does the work; a prime is its own
 * smallest divisor, so the answer doubles as a primality report.
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

export const smallestDivisorAnswers = (): number[] => [199, 1999, 19999].map(smallestDivisor);
