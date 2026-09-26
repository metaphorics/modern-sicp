// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.79: the general second-order solver. Pending scaffold;
 * the solution and its rationale live in solutions/ch3/ex_3_79.ts and
 * .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.79 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The statement's `f`: answers `y''` from `y'` and `y`, in the
 * statement's order (derivative first, position second). */
export type SecondDerivative = (dy: number, y: number) => number;

/** The book's generalized `solve-2nd`: the stream of `y` for
 * `y'' = f(y', y)` from `y0` and `dy0`, integrated at step `dt`. */
export function solveGeneral(
  _f: SecondDerivative,
  _y0: number,
  _dy0: number,
  _dt: number,
): Stream<number> {
  throw new PendingSolution();
}
