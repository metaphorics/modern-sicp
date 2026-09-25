// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Queue } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.21: the print-queue for the pointer-pair queue. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_21.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.21 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Prints the queue as the sequence of its items. */
export function printQueue<A>(_queue: Queue<A>): string {
  throw new PendingSolution();
}

/** Prints the queue's raw representation pair: the front pointer plus
 * the rear pointer labeled. */
export function benPrintQueue<A>(_queue: Queue<A>): string {
  throw new PendingSolution();
}
