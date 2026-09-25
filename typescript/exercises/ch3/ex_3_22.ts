// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 3.22: the queue as a procedure with local state. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_22.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.22 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's dispatch messages for the closure queue. */
export type QueueClosureMessage = "insert!" | "delete!" | "front" | "empty?" | "print";

/** The message-passing queue dispatch, one call per message shape. */
export interface QueueClosure<A> {
  (message: "insert!", item: A): QueueClosure<A>;
  (message: "delete!"): QueueClosure<A>;
  (message: "front"): A;
  (message: "empty?"): boolean;
  (message: "print"): string;
}

/** Builds the closure queue of exercise 3.22. */
export function makeQueueClosure<A>(): QueueClosure<A> {
  throw new PendingSolution();
}
