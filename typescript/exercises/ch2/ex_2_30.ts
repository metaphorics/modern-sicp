// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Tree } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.30: square every leaf of a tree, preserving its shape. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.30 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Squares each leaf by explicit recursion over Leaf and Node. */
export function squareTreeDirect(_t: Tree<number>): Tree<number> {
  throw new PendingSolution();
}

/** Squares each leaf by mapping over the subtrees, leaf case inside. */
export function squareTreeViaMap(_t: Tree<number>): Tree<number> {
  throw new PendingSolution();
}
