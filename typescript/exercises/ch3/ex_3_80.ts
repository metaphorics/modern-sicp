// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.80: the series RLC circuit. Pending scaffold; the
 * solution and its rationale live in solutions/ch3/ex_3_80.ts and
 * .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.80 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The pair the statement's `RLC` answers: the streams of the state
 * variables, in the statement's order (`vC` first, then `iL`). */
export interface RlcStreams {
  readonly vC: Stream<number>;
  readonly iL: Stream<number>;
}

/** The book's `RLC`: takes `R`, `L`, `C` and `dt` and answers a
 * procedure from the initial values `vC0` and `iL0` to the pair of
 * state streams. */
export function RLC(
  _R: number,
  _L: number,
  _C: number,
  _dt: number,
): (vC0: number, iL0: number) => RlcStreams {
  throw new PendingSolution();
}
