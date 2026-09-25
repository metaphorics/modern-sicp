// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.14: the tree the count-change process generates for 11
 * cents, and its orders of growth. The pending artifact is the
 * instrumented count-change that counts the tree's nodes, the data the
 * drawing and the growth claims rest on.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.14 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Ways to change `amount` with `kindsOfCoins` kinds, counting every node visited. */
export function countChangeWithNodes(_amount: number): {
  ways: number;
  nodes: number;
  maxDepth: number;
} {
  throw new PendingSolution();
}
