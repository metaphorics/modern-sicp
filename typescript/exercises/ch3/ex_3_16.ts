// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { MList } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.16: Ben's count-pairs double counts. Pending scaffold;
 * the solution and its rationale live in solutions/ch3/ex_3_16.ts
 * and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.16 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Whether a value is a mutable list: the book's `pair?` over the
 * section's tagged records. */
export function isMListValue(value: unknown): value is MList<unknown> {
  throw new PendingSolution();
}

/** Ben's `count-pairs`: the car count plus the cdr count plus one;
 * double counts shared pairs, never returns on a ring. */
export function countPairs(_x: MList<unknown>): number {
  throw new PendingSolution();
}

/** Three pairs with no sharing, (a b c): Ben's count returns 3. */
export function plainX3(): MList<unknown> {
  throw new PendingSolution();
}

/** Three pairs with one shared pair: Ben's count returns 4. */
export function sharedX2(): MList<unknown> {
  throw new PendingSolution();
}

/** Three pairs, each reachable by two routes: Ben's count returns 7. */
export function sharedX3(): MList<unknown> {
  throw new PendingSolution();
}
