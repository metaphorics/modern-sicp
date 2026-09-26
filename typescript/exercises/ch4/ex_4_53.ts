// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.53: with permanent-set! and if-fail, what is the result of the pairs-accumulation program ending in (amb).
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.53 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_53(): string {
  throw new PendingSolution();
}
