// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { ProbeEvent, Wire } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.31: why accept-action-procedure! runs the action
 * immediately. Pending scaffold; the solution and its rationale live
 * in solutions/ch3/ex_3_31.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.31 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** A wire whose addAction appends without the immediate run. */
export function makeWireWithoutInitRun(): Wire {
  throw new PendingSolution();
}

/** Runs the book's half-adder demo on the module's wires. */
export function demonstrateWithInitRun(): ProbeEvent[] {
  throw new PendingSolution();
}

/** Runs the book's half-adder demo on the no-init wires. */
export function demonstrateWithoutInitRun(): ProbeEvent[] {
  throw new PendingSolution();
}
