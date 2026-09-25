// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Ref } from "effect";

/**
 * Exercise 3.11: where the account state lives, traced over factory
 * frames and balance cells. Pending scaffold; the solution and its
 * rationale live in solutions/ch3/ex_3_11.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.11 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The log a traced factory appends its frame events to. */
export type EnvLog = Ref.Ref<ReadonlyArray<string>>;

/** Builds a traced bank-account object: dispatch plus the balance cell
 * its factory call created. */
export function makeAccountTraced(_initialBalance: number, _label: string, _log: EnvLog): never {
  throw new PendingSolution();
}
