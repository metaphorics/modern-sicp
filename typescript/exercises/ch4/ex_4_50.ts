// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.50: implement the ramb special form, which searches alternatives in a random order, and show how it helps Alyssa's problem.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.50 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_50(): string {
  throw new PendingSolution();
}
