// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.62: div-series over power series and the tangent
 * series. Pending scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_62.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.62 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's `div-series`: s1/s2 as the mul-series of s1 with the
 * inverted s2; throws an `Error` when s2's constant term is 0. */
export function divSeries(_s1: Stream<number>, _s2: Stream<number>): Stream<number> {
  throw new PendingSolution();
}

/** The book's tangent: sin divided by cos. */
export function tangent(): Stream<number> {
  throw new PendingSolution();
}
