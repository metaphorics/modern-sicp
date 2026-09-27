// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Connector } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.34: Louis Reasoner's squarer and the flaw in wiring it
 * as `(multiplier a a b)`. Pending scaffold; the solution and its
 * rationale live in solutions/ch3/ex_3_34.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.34 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Wires Louis's device: `b` constrained to `a * a` by one
 * multiplier whose two factor terminals are the same connector. */
export function squarerFromMultiplier(_a: Connector, _b: Connector): void {
  throw new PendingSolution();
}
