// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Effect, Ref } from "effect";

/**
 * Exercise 3.52: accum traces assignment plus laziness. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_52.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.52 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** One step of the transcript's readings. */
export interface AccumReadings {
  sumAfterSeq: number;
  sumAfterY: number;
  sumAfterZ: number;
  refAnswer: number;
  sumAfterRef: number;
  zDisplayed: number[];
  sumAfterDisplay: number;
}

/** The book's `accum` over the shared `sum` cell. */
export function makeAccum(_sum: Ref.Ref<number>): (x: number) => Effect.Effect<number> {
  throw new PendingSolution();
}

/** The memoized run: the book's answers under `memo-proc`. */
export function memoizedRun(): Effect.Effect<AccumReadings> {
  throw new PendingSolution();
}

/** The plain-thunk run: the exercise's counterfactual. */
export function unmemoizedRun(): Effect.Effect<AccumReadings> {
  throw new PendingSolution();
}
