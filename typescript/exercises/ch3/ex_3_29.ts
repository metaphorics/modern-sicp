// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Agenda, Wire } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.29: the compound or-gate from and-gates and inverters.
 * Pending scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_29.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.29 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Builds the compound or-gate: two inverters feeding an and-gate
 * whose output feeds a final inverter. */
export function orGateFromAnd(_a1: Wire, _a2: Wire, _output: Wire, _agenda: Agenda): void {
  throw new PendingSolution();
}
