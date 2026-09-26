// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.50: the multi-stream stream-map. Pending scaffold; the
 * solution and its rationale live in solutions/ch3/ex_3_50.ts and
 * .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.50 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's `stream-map` completed: element-wise `proc` over any
 * number of streams, ending where the shortest ends. */
export function streamMapN<A, B>(
  _proc: (...args: A[]) => B,
  ..._argstreams: Stream<A>[]
): Stream<B> {
  throw new PendingSolution();
}

/** The book's `add-streams` built on the completed map. */
export function addStreamsN(_s1: Stream<number>, _s2: Stream<number>): Stream<number> {
  throw new PendingSolution();
}

/** The text's check: the integers as the ones plus the integers. */
export function integersThroughMapN(): Stream<number> {
  throw new PendingSolution();
}
