// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.24: design and carry out experiments comparing the speed of
 * the direct metacircular evaluator with the analyzed evaluator, and use
 * the results to estimate the fraction of time spent in analysis versus
 * execution.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.24 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** A recursive, arithmetic-heavy workload for the comparison. */
export const fibSource = `function fib(n: number): number {
  return n < 2 ? n : fib(n - 1) + fib(n - 2);
}
fib(20);`;

export function ex_4_24(): string {
  throw new PendingSolution();
}
