// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.49: Alyssa's generation: change parse-word to always succeed with a word drawn from the word list, and show the first sentences generated.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.49 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_49(): string {
  throw new PendingSolution();
}
