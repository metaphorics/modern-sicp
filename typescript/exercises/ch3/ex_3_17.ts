// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { MList } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.17: counting distinct pairs with a visited set. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_17.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.17 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The number of distinct pairs in any structure: walk head and
 * tail, count a pair only the first time the identity set sees it. */
export function countPairsCorrect(_x: MList<unknown>): number {
  throw new PendingSolution();
}
