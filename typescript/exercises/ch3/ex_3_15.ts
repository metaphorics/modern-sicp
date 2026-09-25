// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { MCons, MList } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.15: set-to-wow! shows sharing. Pending scaffold; the
 * solution and its rationale live in solutions/ch3/ex_3_15.ts and
 * .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.15 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's z1 = (cons x x): one (a b) pair shared by the car and
 * the cdr; the element type is the union because z1 is improper. */
export function makeZ1(): MCons<MList<string> | string> {
  throw new PendingSolution();
}

/** The book's z2: two distinct (a b) copies at the car and the cdr. */
export function makeZ2(): MCons<MList<string> | string> {
  throw new PendingSolution();
}

/** The book's `set-to-wow!`: set-car! of wow on the head pair of
 * `x`, and returns `x`. */
export function setToWow(_x: MCons<MList<string> | string>): MCons<MList<string> | string> {
  throw new PendingSolution();
}
