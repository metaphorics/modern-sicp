// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream, StreamCell } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.65: ln 2 approximation streams. Pending scaffold; the
 * solution and its rationale live in solutions/ch3/ex_3_65.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.65 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's `ln2-summands`: 1/n, then the negation of the rest. */
export function ln2Summands(_n: number): StreamCell<number> {
  throw new PendingSolution();
}

/** The book's `ln2-stream`: the partial sums of the summands. */
export function ln2Stream(): StreamCell<number> {
  throw new PendingSolution();
}

/** The euler transform of the partial sums. */
export function ln2EulerStream(_sums: Stream<number>): Stream<number> {
  throw new PendingSolution();
}

/** The tableau's first column, the accelerated sequence. */
export function ln2AcceleratedStream(_sums: Stream<number>): Stream<number> {
  throw new PendingSolution();
}

/** The limit answer with its examined-element count. */
export interface LimitWithTerms {
  readonly value: number;
  readonly terms: number;
}

export function streamLimitWithTerms(_s: Stream<number>, _tolerance: number): LimitWithTerms {
  throw new PendingSolution();
}
