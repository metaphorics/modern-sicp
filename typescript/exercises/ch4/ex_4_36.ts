// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.36: unbounded Pythagorean triples: why replacing an-integer-between with an-integer-starting-from fails, and a procedure that enumerates all triples under try-again.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.36 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_36(): string {
  throw new PendingSolution();
}
