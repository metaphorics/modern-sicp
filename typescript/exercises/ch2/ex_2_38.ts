// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.38: fold-left, the mirror of the book's fold-right. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.38 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Folds `sequence` left: `op(op(op(initial, x1), x2), x3)`. */
export function foldLeft<A, B>(_op: (x: B, y: A) => B, _initial: B, _sequence: List<A>): B {
  throw new PendingSolution();
}
