// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.6: local bindings are derived expressions. A block binding
 * each name to its initializer is the same as an immediately invoked
 * function over those names. Write the lowering from local bindings to a
 * function call and add it to the evaluator's analysis.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.6 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_06(): string {
  throw new PendingSolution();
}
