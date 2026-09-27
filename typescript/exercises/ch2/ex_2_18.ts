// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.18: reverse a list. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.18 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The elements of `items` in reverse order. */
export function reverse(_items: List<number>): List<number> {
  throw new PendingSolution();
}
