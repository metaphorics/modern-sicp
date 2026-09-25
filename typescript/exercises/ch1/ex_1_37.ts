// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.37: contFrac evaluates the k-term finite continued fraction
 * with numerators n(i) and denominators d(i). The iterative variant is a
 * loop folding the fraction up from term k (no tail-call guarantee on
 * Node), and the artifact reports how large k must be for 4-decimal
 * accuracy of 1/phi.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.37 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The k-term continued fraction in the section's recursive shape. */
export function contFrac(_n: (i: number) => number, _d: (i: number) => number, _k: number): number {
  throw new PendingSolution();
}

/** The same continued fraction as a while loop over the terms. */
export function contFracIter(
  _n: (i: number) => number,
  _d: (i: number) => number,
  _k: number,
): number {
  throw new PendingSolution();
}

/** The smallest k whose contFrac all-ones value is accurate to 4 decimal places. */
export function smallestKForFourDecimals(): number {
  throw new PendingSolution();
}
