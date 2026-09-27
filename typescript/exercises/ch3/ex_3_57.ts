// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { StreamCell } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.57: additions performed computing fibs elements. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_57.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.57 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** A tally the instrumented streams report additions into. */
export interface AdditionCounter {
  additions: number;
}

/** The text's add-streams fibs, instrumented, memoized tails. */
export function fibsCounting(_counter: AdditionCounter): StreamCell<number> {
  throw new PendingSolution();
}

/** The book's cons-stream with delay spelled as a plain thunk. */
export interface PlainStreamCell<A> {
  readonly head: A;
  readonly tail: () => PlainStream<A>;
}

/** A plain-thunk stream: tails recompute on every reference. */
export type PlainStream<A> = PlainStreamCell<A> | null;

/** The same instrumented fibs built on plain thunks. */
export function fibsPlainCounting(_counter: AdditionCounter): PlainStreamCell<number> {
  throw new PendingSolution();
}
