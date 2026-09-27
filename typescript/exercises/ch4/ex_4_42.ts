// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.42: solve the Liars puzzle: each of five girls makes one true and one untrue statement; find the real order of placement.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.42 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_42(): string {
  throw new PendingSolution();
}
