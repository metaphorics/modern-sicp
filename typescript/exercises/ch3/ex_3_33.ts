// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Connector, Constraint } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.33: an averager over three connectors, the constraint
 * that c is the average of a and b. Pending scaffold; the solution
 * and its rationale live in solutions/ch3/ex_3_33.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.33 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Constrains `c` to be the average of `a` and `b`, all three
 * directions. */
export function averager(_a: Connector, _b: Connector, _c: Connector): Constraint {
  throw new PendingSolution();
}
