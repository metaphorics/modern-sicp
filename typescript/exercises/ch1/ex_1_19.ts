// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.19: Fibonacci in a logarithmic number of steps by
 * squaring the T_pq transformation. Fib(93) already exceeds number's
 * exact-integer range, so fibLog computes over bigint.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.19 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Fib(n) over bigint in a logarithmic number of steps. */
export function fibLog(_n: number): bigint {
  throw new PendingSolution();
}
