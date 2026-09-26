// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { StreamCell } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.63: why sqrt-stream localizes its guesses. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_63.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.63 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** One Newton step for sqrt(x), parameterized so it can be counted. */
export type SqrtImprove = (guess: number, x: number) => number;

/** A sqrt-stream builder in one of the exercise's shapes. */
export type SqrtStreamMaker = (x: number, improve: SqrtImprove) => StreamCell<number>;

/** Alyssa's shape: the local `guesses` binding, memoized tails. */
export function sqrtStreamLocal(_x: number, _improve: SqrtImprove): StreamCell<number> {
  throw new PendingSolution();
}

/** Louis's shape: the map over a fresh recursive call. */
export function sqrtStreamExternal(_x: number, _improve: SqrtImprove): StreamCell<number> {
  throw new PendingSolution();
}

/** Alyssa's shape with plain-lambda tails, no memo-proc. */
export function sqrtStreamUnmemoizedLocal(_x: number, _improve: SqrtImprove): StreamCell<number> {
  throw new PendingSolution();
}

/** Louis's shape with plain-lambda tails. */
export function sqrtStreamUnmemoizedExternal(
  _x: number,
  _improve: SqrtImprove,
): StreamCell<number> {
  throw new PendingSolution();
}
