// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.33: filteredAccumulate.
 *
 * The abstraction is accumulate with one extra argument: values of the range
 * whose predicate fails contribute the null value instead of their term.
 * The primes predicate and gcd are the section's own tools of 1.2.2 and
 * 1.2.5, restated here to keep the solution self-contained.
 */
export function filteredAccumulate(
  combiner: (x: number, y: number) => number,
  nullValue: number,
  term: (x: number) => number,
  a: number,
  next: (x: number) => number,
  b: number,
  filter: (x: number) => boolean,
): number {
  if (a > b) {
    return nullValue;
  }
  const rest = filteredAccumulate(combiner, nullValue, term, next(a), next, b, filter);
  return filter(a) ? combiner(term(a), rest) : rest;
}

/** The smallest-divisor primality test of 1.2.2, as a divisor loop. */
export function isPrime(n: number): boolean {
  if (n < 2) {
    return false;
  }
  let d = 2;
  while (d * d <= n) {
    if (n % d === 0) {
      return false;
    }
    d += 1;
  }
  return true;
}

/** Euclid's Algorithm of 1.2.5, as a loop. */
export function gcd(a: number, b: number): number {
  let x = a;
  let y = b;
  while (y !== 0) {
    const r = x % y;
    x = y;
    y = r;
  }
  return x;
}

/** The sum of the squares of the primes in the interval a..b. */
export function sumOfSquaresOfPrimes(a: number, b: number): number {
  return filteredAccumulate(
    (x, y) => x + y,
    0,
    (x) => x * x,
    a,
    (x) => x + 1,
    b,
    isPrime,
  );
}

/** The product of the positive integers i < n with gcd(i, n) = 1. */
export function productOfRelativelyPrimes(n: number): number {
  return filteredAccumulate(
    (x, y) => x * y,
    1,
    (x) => x,
    1,
    (x) => x + 1,
    n - 1,
    (i) => gcd(i, n) === 1,
  );
}
