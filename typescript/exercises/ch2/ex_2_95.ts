// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.95: where integer arithmetic fails the gcd: The book's P1, P2, P3
 * products share P1, but truncating integer division answers the first
 * leading-coefficient division with zero. Pending scaffold; the solution and
 * its rationale live in solutions/ch2/ex_2_95.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.95 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Placeholder: throws until the solution lands. */
export function scaffold(): never {
  throw new PendingSolution();
}
