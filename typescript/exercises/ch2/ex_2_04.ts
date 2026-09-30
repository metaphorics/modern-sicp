// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.4: the alternative procedural representation of pairs, where
 * consPair returns a function that hands both parts to a selector. The
 * statement's carPair and the corresponding cdrPair must satisfy the pair law.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.4 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The function pair type: a pair hands both of its parts to a selector. */
export type Pair<A, B> = <R>(m: (x: A, y: B) => R) => R;

/** The procedural cons: both parts close over x and y. */
export function consPair<A, B>(_x: A, _y: B): Pair<A, B> {
  throw new PendingSolution();
}

/** The selector that recovers the first part. */
export function carPair<A, B>(_z: Pair<A, B>): A {
  throw new PendingSolution();
}

/** The corresponding selector for the second part. */
export function cdrPair<A, B>(_z: Pair<A, B>): B {
  throw new PendingSolution();
}
