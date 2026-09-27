// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.83: a generic raise for the tower: Each level but the top
 * installs a one-step raise: ordinary numbers to rationals, rationals to
 * reals, reals to complex numbers. Pending scaffold; the solution and its
 * rationale live in solutions/ch2/ex_2_83.ts and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.83 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Placeholder: throws until the solution lands. */
export function scaffold(): never {
  throw new PendingSolution();
}
