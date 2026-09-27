// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.32: give examples that illustrate the difference between the
 * streams of chapter 3 and the lazier lazy lists of this section, where both
 * the car and the cdr of a pair are delayed. How can the extra laziness be
 * used?
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.32 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_32(): string {
  throw new PendingSolution();
}
