// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.32: accumulate combines a collection of terms with a general
 * two-argument combiner and a null value; sum and product are simple calls
 * to it. The iterative variant is a loop (no tail-call guarantee on Node).
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.32 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The recursive accumulate: combiner(term(a), accumulate(rest)). */
export function accumulate(
  _combiner: (x: number, y: number) => number,
  _nullValue: number,
  _term: (x: number) => number,
  _a: number,
  _next: (x: number) => number,
  _b: number,
): number {
  throw new PendingSolution();
}

/** The same accumulate as a while loop carrying the accumulation. */
export function accumulateIter(
  _combiner: (x: number, y: number) => number,
  _nullValue: number,
  _term: (x: number) => number,
  _a: number,
  _next: (x: number) => number,
  _b: number,
): number {
  throw new PendingSolution();
}
