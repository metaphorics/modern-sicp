// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.54: mul-streams and the factorial stream. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_54.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.54 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's `mul-streams`: element-wise product of two streams. */
export function mulStreams(_s1: Stream<number>, _s2: Stream<number>): Stream<number> {
  throw new PendingSolution();
}

/** The statement's completed definition: element n counting from 0
 * is (n + 1)!. */
export function factorials(): Stream<number> {
  throw new PendingSolution();
}
