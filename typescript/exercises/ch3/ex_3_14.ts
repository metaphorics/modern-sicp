// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { MList } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.14: mystery reverses a list. Pending scaffold; the
 * solution and its rationale live in solutions/ch3/ex_3_14.ts and
 * .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.14 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's `mystery`: `x` reversed by rewriting the pairs' tails
 * in place with `setCdr`; the caller's binding is left holding its
 * first pair, whose tail is now empty. */
export function mystery<A>(_x: MList<A>): MList<A> {
  throw new PendingSolution();
}
