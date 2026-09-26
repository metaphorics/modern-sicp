// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.64: stream-limit. Pending scaffold; the solution and its
 * rationale live in solutions/ch3/ex_3_64.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.64 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's `stream-limit`: the first successor within the
 * tolerance of its predecessor. */
export function streamLimit(_s: Stream<number>, _tolerance: number): number {
  throw new PendingSolution();
}

/** The exercise's `sqrt` built on the module's Newton stream. */
export function sqrtWithin(_x: number, _tolerance: number): number {
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

/** Newton's stream pushed through the euler tableau. */
export function acceleratedSqrtStream(_x: number): Stream<number> {
  throw new PendingSolution();
}
