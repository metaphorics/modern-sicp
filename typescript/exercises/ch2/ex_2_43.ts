// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.43: Louis Reasoner's eight-queens, mappings interchanged. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.43 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Louis's queens: the (k-1)-solutions are recomputed for every row. */
export function queensSwapped(_boardSize: number): List<List<number>> {
  throw new PendingSolution();
}
