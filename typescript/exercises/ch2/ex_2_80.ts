// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.80: a generic =zero?: Each package answers zero for its own
 * arguments, dispatching on the tag; exercise 2.87 extends the same operation
 * to polynomials. Pending scaffold; the solution and its rationale live in
 * solutions/ch2/ex_2_80.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.80 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Placeholder: throws until the solution lands. */
export function scaffold(): never {
  throw new PendingSolution();
}
