// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.41: write an ordinary program, in the working language, that solves the multiple dwelling puzzle.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.41 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_41(): string {
  throw new PendingSolution();
}
