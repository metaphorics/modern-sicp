// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.61: invert-unit-series from X = 1 - S_R * X. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_61.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.61 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's `invert-unit-series`: the series X with constant term 1
 * whose remainder is the negation of S_R (the part of s after the
 * constant term) times X itself. */
export function invertUnitSeries(_s: Stream<number>): Stream<number> {
  throw new PendingSolution();
}
