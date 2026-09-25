// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Tree } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.35: count-leaves as an accumulation. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.35 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Counts the leaves of `t` as an accumulation over its fringe. */
export function countLeavesViaAccumulate(_t: Tree<number>): number {
  throw new PendingSolution();
}
