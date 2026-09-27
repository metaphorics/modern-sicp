// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.39: reverse via fold-right and fold-left. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.39 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Reverses `sequence` by folding right, appending each element last. */
export function reverseViaFoldRight(_sequence: List<number>): List<number> {
  throw new PendingSolution();
}

/** Reverses `sequence` by folding left, consing each element in front. */
export function reverseViaFoldLeft(_sequence: List<number>): List<number> {
  throw new PendingSolution();
}
