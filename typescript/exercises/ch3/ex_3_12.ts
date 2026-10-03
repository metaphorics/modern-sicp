// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { MCons, MList } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.12: append copies, appendBang splices. Pending scaffold;
 * the solution and its rationale live in solutions/ch3/ex_3_12.ts
 * and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.12 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's `append`: a fresh copy of `x` followed by the very
 * `y` object, leaving `x` unchanged. */
export function append<A>(_x: MList<A>, _y: MList<A>): MList<A> {
  throw new PendingSolution();
}

/** The book's `last-pair`: the final pair of a nonempty list. */
export function lastPair<A>(_x: MList<A>): MCons<A> {
  throw new PendingSolution();
}

/** The book's `appendBang`: rewrites the final pair of `x` to point at
 * `y` and returns `x`; empty `x` throws. */
export function appendBang<A>(_x: MList<A>, _y: MList<A>): MList<A> {
  throw new PendingSolution();
}

/** The four printed shapes of the book's interaction, plus the
 * identity pin on the final pair of `x`. */
export interface AppendDemo {
  readonly zAfterAppend: string;
  readonly cdrXAfterAppend: string;
  readonly wAfterAppendBang: string;
  readonly cdrXAfterAppendBang: string;
  readonly xTailIsY: boolean;
}

/** Runs the book's append / appendBang interaction over x = (a b), y =
 * (c d) and reports the pins. */
export function appendDemo(): AppendDemo {
  throw new PendingSolution();
}
