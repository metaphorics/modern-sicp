// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.38: modify multiple-dwelling to omit the Smith-Fletcher adjacency requirement; how many solutions does the modified puzzle have.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.38 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_38(): string {
  throw new PendingSolution();
}
