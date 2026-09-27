// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { MList } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.19: constant-space cycle detection. Pending scaffold;
 * the solution and its rationale live in solutions/ch3/ex_3_19.ts
 * and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.19 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Whether the tail chase from `x` ever revisits a pair, decided in
 * constant space by tortoise and hare. */
export function hasCycleConstantSpace(_x: MList<unknown>): boolean {
  throw new PendingSolution();
}
