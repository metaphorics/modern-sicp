// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List, Tree } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.28: fringe, the leaves of a tree left to right. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.28 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The leaves of `t`, left to right. */
export function fringe(_t: Tree<number>): List<number> {
  throw new PendingSolution();
}
