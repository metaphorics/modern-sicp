// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.32: the set of all subsets of a set. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.32 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** All subsets of `s`, in the book's doubling order. */
export function subsets(_s: List<number>): List<List<number>> {
  throw new PendingSolution();
}
