// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.84: coercion by successive raising: The lower argument is raised
 * until its type matches the other's, or the attempt runs off the top of the
 * tower. Pending scaffold; the solution and its rationale live in
 * solutions/ch2/ex_2_84.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.84 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Placeholder: throws until the solution lands. */
export function scaffold(): never {
  throw new PendingSolution();
}
