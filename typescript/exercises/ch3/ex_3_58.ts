// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Stream } from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.58: the long-division digit stream of expand. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_58.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.58 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's `expand`: the successive digits of num/den in the
 * radix, one per element, by repeated long division. */
export function expand(_num: number, _den: number, _radix: number): Stream<number> {
  throw new PendingSolution();
}
