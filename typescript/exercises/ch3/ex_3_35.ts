// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Connector, Constraint } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.35: Ben Bitdiddle's squarer as a new primitive
 * constraint, the book's outline filled in. Pending scaffold; the
 * solution and its rationale live in solutions/ch3/ex_3_35.ts and
 * .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.35 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Constrains `b` to be the square of `a`, both directions, refusing
 * a negative `b`. */
export function squarer(_a: Connector, _b: Connector): Constraint {
  throw new PendingSolution();
}
