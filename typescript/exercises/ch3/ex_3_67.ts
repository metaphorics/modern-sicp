// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.67: all pairs of two streams. Pending scaffold; the
 * solution and its rationale live in solutions/ch3/ex_3_67.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.67 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's modified `pairs`: every pair of S and T, both orders. */
export function allPairs<A>(_s: Stream<A>, _t: Stream<A>): Stream<[A, A]> {
  throw new PendingSolution();
}

/** The exercise-3.66 position walker, restated. */
export function positionOfPair(
  _s: Stream<[number, number]>,
  _target: readonly [number, number],
  _limit: number,
): number {
  throw new PendingSolution();
}
