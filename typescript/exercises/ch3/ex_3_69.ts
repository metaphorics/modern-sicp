// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.69: write a procedure `triples` that takes three infinite
 * streams S, T, and U and produces the stream of triples (Si, Tj, Uk)
 * such that i <= j <= k. Use `triples` to generate the stream of all
 * Pythagorean triples of positive integers, the triples (i, j, k)
 * such that i <= j and i^2 + j^2 = k^2. Pending scaffold; the solution
 * and its rationale live in solutions/ch3/ex_3_69.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.69 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** One triple of integers. */
export type Triple = [number, number, number];

/** The statement's `triples`: (Si, Tj, Uk) with i <= j <= k. */
export function triples(
  _s: Stream<number>,
  _t: Stream<number>,
  _u: Stream<number>,
): Stream<Triple> {
  throw new PendingSolution();
}

/** The statement's Pythagorean triples: the triples of the integers
 * whose i^2 + j^2 = k^2. */
export const pythagoreanTriples: Stream<Triple> = (() => {
  throw new PendingSolution();
})();
