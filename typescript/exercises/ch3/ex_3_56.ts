// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.56: the Hamming numbers via merge. Pending scaffold; the
 * solution and its rationale live in solutions/ch3/ex_3_56.ts and
 * .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.56 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The statement's S: the positive integers with no prime factors
 * other than 2, 3, or 5, in ascending order with no repetitions. */
export function S(): Stream<number> {
  throw new PendingSolution();
}
