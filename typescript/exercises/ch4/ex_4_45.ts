// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.45: the sentence 'The professor lectures to the student in the class with the cat' parses five ways; give the parses and their shades of meaning.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.45 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_45(): string {
  throw new PendingSolution();
}
