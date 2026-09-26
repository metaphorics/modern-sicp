// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.82: Monte Carlo integration as streams. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_82.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.82 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** One experiment point: two random states mapped uniformly onto the
 * rectangle [x1, x2] x [y1, y2]. */
export interface Point {
  readonly x: number;
  readonly y: number;
}

/** The book's "random points in the rectangle": the stream of points
 * drawn from the module's `randomNumbers`, two generator states per
 * point, mapped onto the given rectangle. */
export function randomPointsIn(_x1: number, _x2: number, _y1: number, _y2: number): Stream<Point> {
  throw new PendingSolution();
}

/** The book's `estimate-integral`, stream edition: the stream of
 * Monte Carlo estimates of the area where `pred` holds inside the
 * rectangle, each estimate the passing fraction times the
 * rectangle's area. */
export function estimateIntegral(
  _pred: (x: number, y: number) => boolean,
  _x1: number,
  _x2: number,
  _y1: number,
  _y2: number,
): Stream<number> {
  throw new PendingSolution();
}

/** Exercise 3.5's trial run: the estimates for the unit circle inside
 * the square [-1, 1] x [-1, 1], homing in on pi. */
export function unitCircleEstimates(): Stream<number> {
  throw new PendingSolution();
}
