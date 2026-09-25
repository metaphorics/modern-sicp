// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.21: square-list in two spellings. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.21 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The explicit spelling: square of the head consed onto the mapped tail. */
export function squareListDirect(_items: List<number>): List<number> {
  throw new PendingSolution();
}

/** The map spelling: square lifted over the list. */
export function squareListViaMap(_items: List<number>): List<number> {
  throw new PendingSolution();
}
