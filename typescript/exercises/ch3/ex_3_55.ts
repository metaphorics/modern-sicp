// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.55: partial-sums. Pending scaffold; the solution and its
 * rationale live in solutions/ch3/ex_3_55.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.55 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's `partial-sums`: the running total of the elements. */
export function partialSumsEx(_s: Stream<number>): Stream<number> {
  throw new PendingSolution();
}
