// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.11: f(n) = n for n < 3, f(n) = f(n-1) + 2f(n-2) + 3f(n-3)
 * otherwise. The pending artifact is the pair of functions computing f.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.11 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Computes f by means of a recursive process. */
export function fRecursive(_n: number): number {
  throw new PendingSolution();
}

/** Computes f by means of an iterative process (an explicit loop in this edition). */
export function fIter(_n: number): number {
  throw new PendingSolution();
}
