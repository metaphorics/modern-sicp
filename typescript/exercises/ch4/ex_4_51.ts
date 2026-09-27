// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.51: implement permanent-set!, an assignment that is not undone upon failure, and give the values the count example displays.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.51 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_51(): string {
  throw new PendingSolution();
}
