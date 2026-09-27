// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.75: Louis's buggy smoothing detector fix. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_75.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.75 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Louis's version, exactly as the statement prints it: `avpt`
 * averages the raw sample with the carried slot, the detector
 * compares `avpt` against that same slot, and the recursion carries
 * `avpt` on in place of the raw previous value. */
export function louisZeroCrossings(
  _inputStream: Stream<number>,
  _lastValue: number,
): Stream<number> {
  throw new PendingSolution();
}

/** The fix: Louis's structure with the hint's extra argument, the
 * previous smoothed value, carried separately from the previous raw
 * sample. */
export function makeZeroCrossingsSmoothed(
  _inputStream: Stream<number>,
  _lastValue: number,
  _lastAvpt: number,
): Stream<number> {
  throw new PendingSolution();
}

/** Alyssa's plan assembled: the zero crossings of the
 * pairwise-smoothed signal, seeded at 0. */
export function zeroCrossingsSmoothed(_s: Stream<number>): Stream<number> {
  throw new PendingSolution();
}
