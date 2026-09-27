// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.10: Ackermann's function. ackermann is given by the
 * statement; the pending artifact is the set of wrapper procedures whose
 * mathematical definitions the exercise asks for.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.10 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Ackermann's function as the statement defines it. */
export function ackermann(_x: number, _y: number): number {
  throw new PendingSolution();
}
