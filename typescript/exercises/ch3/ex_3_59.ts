// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.59: integrate-series and the exp, sine, cosine
 * coefficient streams. Pending scaffold; the solution and its
 * rationale live in solutions/ch3/ex_3_59.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.59 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's `integrate-series`: the non-constant coefficients of
 * the integral, a0, a1/2, a2/3, .... */
export function integrateSeries(_s: Stream<number>): Stream<number> {
  throw new PendingSolution();
}

/** The book's `exp-series`: 1 consed onto the integral of itself. */
export function expSeries(): Stream<number> {
  throw new PendingSolution();
}

/** The book's `cosine-series`: constant term cos(0) = 1, then the
 * integral of -sin. */
export function cosineSeries(): Stream<number> {
  throw new PendingSolution();
}

/** The book's `sine-series`: constant term sin(0) = 0, then the
 * integral of cos. */
export function sineSeries(): Stream<number> {
  throw new PendingSolution();
}
