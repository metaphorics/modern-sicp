// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.41: ordered triples of distinct integers with a fixed sum. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.41 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The ordered triples (i j k) of distinct 1..n integers summing to `s`. */
export function orderedTriples(_n: number, _s: number): List<List<number>> {
  throw new PendingSolution();
}
