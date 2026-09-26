// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.60: the mul-series convolution completing the book's
 * cons-stream/add-streams skeleton. Pending scaffold; the solution
 * and its rationale live in solutions/ch3/ex_3_60.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.60 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's `mul-series` completed: a0*b0 consed onto the cross
 * terms, the tail of s1 scaled by b0 plus the tail-of-s1 by s2
 * product. */
export function mulSeries(_s1: Stream<number>, _s2: Stream<number>): Stream<number> {
  throw new PendingSolution();
}
