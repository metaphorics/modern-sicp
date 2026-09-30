// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.73: the RC circuit as a signal processor. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_73.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.73 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's `rc`: takes R, C, and dt and answers a procedure from
 * the current stream and the initial capacitor voltage v0 to the
 * voltage stream. */
export function rc(
  _R: number,
  _C: number,
  _dt: number,
): (current: Stream<number>, v0: number) => Stream<number> {
  throw new PendingSolution();
}

/** The statement's example: R = 5 ohms, C = 1 farad, dt = 0.5 s. */
export function rc1(_current: Stream<number>, _v0: number): Stream<number> {
  throw new PendingSolution();
}
