// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 3.39: which of the five possibilities remain when the
 * square's computation runs under the serializer but its `set!` does
 * not, while the increment is serialized whole. Pending scaffold; the
 * solution and its rationale live in solutions/ch3/ex_3_39.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.39 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The shared cell holding the book's `x`. */
export interface XCell {
  x: number;
}

/** Runs the exercise's two processes in every schedule and answers the
 * distinct final values of `x`, ascending. */
export function exercise39Outcomes(): ReadonlyArray<number> {
  throw new PendingSolution();
}
