// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Tree } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.27: deep-reverse for the tree union. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.27 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Reverses the subtree order at every level. */
export function deepReverse(_t: Tree<number>): Tree<number> {
  throw new PendingSolution();
}

/** Reverses only the outermost subtree order. */
export function shallowReverse(_t: Tree<number>): Tree<number> {
  throw new PendingSolution();
}
