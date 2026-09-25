// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.23: for-each, map's cousin that returns nothing useful. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.23 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Applies `f` to every element left to right; returns true. */
export function forEach<A>(_f: (x: A) => void, _items: List<A>): boolean {
  throw new PendingSolution();
}
