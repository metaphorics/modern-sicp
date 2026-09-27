// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { MCons, MList } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.13: make-cycle builds a ring. Pending scaffold; the
 * solution and its rationale live in solutions/ch3/ex_3_13.ts and
 * .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.13 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's `make-cycle`: mutates the final pair of `x` so its
 * tail is `x` itself, and returns `x`. The ring must never be
 * printed: `showMList` on it never terminates. */
export function makeCycle<A>(_x: MList<A>): MCons<A> {
  throw new PendingSolution();
}

/** The identity pins of the book's ring over (a b c). */
export interface CycleDemo {
  readonly firstHead: string;
  readonly secondHead: string;
  readonly thirdHead: string;
  readonly thirdTailsBackToFirst: boolean;
}

/** Walks the book's ring and reports the heads plus the identity
 * pin; pins by identity, never by printing. */
export function cycleDemo(): CycleDemo {
  throw new PendingSolution();
}
