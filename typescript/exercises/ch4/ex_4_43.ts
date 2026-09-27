// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.43: the yacht puzzle: name Lorna's father, efficiently, and determine how many solutions exist when Mary Ann's surname is not given.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.43 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_43(): string {
  throw new PendingSolution();
}
