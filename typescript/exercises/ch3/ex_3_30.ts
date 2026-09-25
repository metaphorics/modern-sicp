// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Agenda, Wire } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.30: the ripple-carry adder. Pending scaffold; the
 * solution and its rationale live in solutions/ch3/ex_3_30.ts and
 * .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.30 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Builds the ripple-carry adder over LSB-first wire lists and
 * returns the most significant carry-out. */
export function rippleCarryAdder(
  _listA: Wire[],
  _listB: Wire[],
  _listS: Wire[],
  _cIn: Wire,
  _agenda: Agenda,
): Wire {
  throw new PendingSolution();
}
