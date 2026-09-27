// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.66: the order of the pairs stream. Pending scaffold; the
 * solution and its rationale live in solutions/ch3/ex_3_66.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.66 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The stream the statement examines: (pairs integers integers). */
export function intPairs(): Stream<[number, number]> {
  throw new PendingSolution();
}

/** The 0-based position of the first pair equal to `target`, or -1
 * within the first `limit` elements. */
export function positionOfPair(
  _s: Stream<[number, number]>,
  _target: readonly [number, number],
  _limit: number,
): number {
  throw new PendingSolution();
}
