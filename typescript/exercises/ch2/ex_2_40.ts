// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";

/** Exercise 2.40: unique-pairs, so prime-sum-pairs reads as its composition. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.40 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The pairs (i j) with 1 <= j < i <= n, i descending outermost. */
export function uniquePairs(_n: number): List<List<number>> {
  throw new PendingSolution();
}

/** The book's prime-sum-pairs, assembled from unique-pairs. */
export function primeSumPairsSimplified(_n: number): List<List<number>> {
  throw new PendingSolution();
}
