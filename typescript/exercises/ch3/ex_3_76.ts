// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.76: smooth as a reusable component. Pending scaffold;
 * the solution and its rationale live in solutions/ch3/ex_3_76.ts
 * and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.76 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's `smooth`: each element is the average of two successive
 * elements of `s`. */
export function smooth(_s: Stream<number>): Stream<number> {
  throw new PendingSolution();
}

/** The zero-crossing detector rewritten from components: the detector
 * over consecutive smoothed points. */
export function zeroCrossings(_s: Stream<number>): Stream<number> {
  throw new PendingSolution();
}
