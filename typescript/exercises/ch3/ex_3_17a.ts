// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { MList } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.17a: counting distinct nodes. Pending scaffold; the
 * solution and its rationale live in solutions/ch3/ex_3_17a.ts and
 * .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.17a is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The number of distinct nodes in a structure: pairs deduplicated
 * by identity, leaf atoms by value, walking head and tail from the
 * root. */
export function countNodes(_x: MList<unknown>): number {
  throw new PendingSolution();
}
