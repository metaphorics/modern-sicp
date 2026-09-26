// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.29: exhibit a program that runs much more slowly without
 * memoization than with it, and give the responses of the (square (id 10))
 * interaction both when the evaluator memoizes and when it does not.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.29 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_29(): string {
  throw new PendingSolution();
}
