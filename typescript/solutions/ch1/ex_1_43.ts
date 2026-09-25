// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.43: n-fold repeated application.
 *
 * repeated(f, n) is the function that applies f n times, so
 * (repeated square 2) is square composed with itself. The statement
 * suggests exercise 1.42's compose: each step wraps f one more time
 * around the (n-1)-fold application, innermost application first. The
 * composition is spelled inline rather than re-imported, keeping the
 * solution self-contained like its neighbors.
 */
export function repeated(f: (x: number) => number, n: number): (x: number) => number {
  return n <= 1 ? f : (x: number): number => f(repeated(f, n - 1)(x));
}
