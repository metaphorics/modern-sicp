// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Effect } from "effect";

/**
 * Exercise 3.5: Monte Carlo integration over a rectangle, estimating pi
 * from the unit circle. Pending scaffold; the solution and its
 * rationale live in solutions/ch3/ex_3_05.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.5 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Estimates the region's area inside the rectangle by random points. */
export function estimateIntegral(
  _predicate: (x: number, y: number) => boolean,
  _x1: number,
  _x2: number,
  _y1: number,
  _y2: number,
  _trials: number,
  _rand: Effect.Effect<number>,
): never {
  throw new PendingSolution();
}
