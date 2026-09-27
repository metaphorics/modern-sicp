// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.44: write a nondeterministic program for the eight-queens puzzle of exercise 2.42.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.44 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_44(): string {
  throw new PendingSolution();
}
