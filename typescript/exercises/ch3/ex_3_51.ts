// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.51: show reveals memoized delay timing. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_51.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.51 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's `show`: renders its argument into the transcript and
 * returns the argument unchanged. */
export function show<A>(_x: A, _transcript: string[]): A {
  throw new PendingSolution();
}

/** The book's `x`: show mapped over the interval 0 through 10. */
export function makeX(_transcript: string[]): Stream<number> {
  throw new PendingSolution();
}

/** The counterfactual: a map built with plain thunk tails, without
 * `memo-proc`. */
export function makeUnmemoizedX(_transcript: string[]): Stream<number> {
  throw new PendingSolution();
}
