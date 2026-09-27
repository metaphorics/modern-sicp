// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Tree } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.31: abstract tree-map out of square-tree. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.31 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Applies `f` to every leaf of `t`, preserving the tree's shape. */
export function treeMap(_f: (x: number) => number, _t: Tree<number>): Tree<number> {
  throw new PendingSolution();
}
