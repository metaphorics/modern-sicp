// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 3.9: environment structures of the two factorials, measured
 * as call-frame depths. Pending scaffold; the solution and its
 * rationale live in solutions/ch3/ex_3_09.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.9 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Traces one run of the recursive factorial: result and deepest frame
 * count reached. */
export function factorialTraced(_n: number): never {
  throw new PendingSolution();
}
