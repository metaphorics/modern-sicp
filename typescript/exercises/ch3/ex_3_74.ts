// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.74: zero crossings via the generalized stream-map.
 * Pending scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_74.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.74 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's `sense-data`: the sensor signal the text displays as
 * ... 1 2 1.5 1 0.5 -0.1 -2 -3 -2 -0.5 0.2 3 4 ... . */
export function senseData(): Stream<number> {
  throw new PendingSolution();
}

/** The book's completed `zero-crossings`: `sign-change-detector` over
 * the input and the same input delayed by one sample, 0 padded in
 * front. */
export function zeroCrossings(_s: Stream<number>): Stream<number> {
  throw new PendingSolution();
}
