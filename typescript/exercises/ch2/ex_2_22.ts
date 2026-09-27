// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List, Tree } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.22: two iterative square-list rewrites and why they fail. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.22 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Louis's first rewrite: the right process, reversed order. */
export function squareListIter(_items: List<number>): List<number> {
  throw new PendingSolution();
}

/** The second rewrite's shape with the cons arguments interchanged. */
export function squareListSwapped(_items: List<number>): Tree<number> {
  throw new PendingSolution();
}
