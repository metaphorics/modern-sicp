// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.78: `solve-2nd`, the second-order feedback loop. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_78.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.78 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's `solve-2nd`: the stream of `y` for `y'' = a y' + b y`
 * from `y0` and `dy0`, integrated at step `dt`. */
export function solve2nd(
  _a: number,
  _b: number,
  _y0: number,
  _dy0: number,
  _dt: number,
): Stream<number> {
  throw new PendingSolution();
}
