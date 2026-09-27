// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { MList } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.18: whether a list contains a cycle. Pending scaffold;
 * the solution and its rationale live in solutions/ch3/ex_3_18.ts
 * and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.18 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Whether following the tails of `x` ever revisits a pair. */
export function hasCycle(_x: MList<unknown>): boolean {
  throw new PendingSolution();
}
