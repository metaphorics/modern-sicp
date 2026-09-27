// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 3.20: trace object aliasing via closures. Pending
 * scaffold; the solution and its rationale live in
 * solutions/ch3/ex_3_20.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.20 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The book's procedural pair: a dispatch over two local slots. */
export interface MutablePair<A, B> {
  readonly getX: () => A;
  readonly getY: () => B;
  readonly setX: (value: A) => void;
  readonly setY: (value: B) => void;
}

/** Builds the book's procedural `cons` over two local slots. */
export function makePair<A, B>(_x: A, _y: B): MutablePair<A, B> {
  throw new PendingSolution();
}
