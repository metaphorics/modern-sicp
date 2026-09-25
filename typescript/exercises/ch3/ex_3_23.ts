// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 3.23: the deque with constant-time ends. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_23.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.23 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** One doubly linked cell: the item plus the two links. */
export interface DequeNode<A> {
  item: A;
  prev: DequeNode<A> | null;
  next: DequeNode<A> | null;
}

/** The deque: front and rear pointers over the doubly linked cells. */
export interface Deque<A> {
  front: DequeNode<A> | null;
  rear: DequeNode<A> | null;
}

/** Builds an empty deque. */
export function makeDeque<A>(): Deque<A> {
  throw new PendingSolution();
}
