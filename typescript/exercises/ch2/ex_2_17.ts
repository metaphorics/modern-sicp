// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.17: the last pair of a list. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.17 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Returns the list that contains only the last element of `items`. */
export function lastPair(_items: List<number>): List<number> {
  throw new PendingSolution();
}
