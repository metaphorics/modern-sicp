// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Agenda, Wire } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.28: the or-gate as a primitive function box. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_28.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.28 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The exercise's own or function: the book's `logical-or`. */
export function logicalOrExercise(_a: boolean, _b: boolean): boolean {
  throw new PendingSolution();
}

/** Connects `output` to the or of the two inputs, one `orGateDelay`
 * after either input changes. */
export function orGatePrimitive(_a1: Wire, _a2: Wire, _output: Wire, _agenda: Agenda): void {
  throw new PendingSolution();
}
