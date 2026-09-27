// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.81: Louis Reasoner's self-coercions and the guard: Identity
 * coercion entries, the infinite loop they cause, and the guard: arguments
 * that already share a type are never coerced. Pending scaffold; the solution
 * and its rationale live in solutions/ch2/ex_2_81.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.81 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Placeholder: throws until the solution lands. */
export function scaffold(): never {
  throw new PendingSolution();
}
