// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.46: the amb evaluator evaluates operands left to right; explain why the parsing program would fail under another order.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.46 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_46(): string {
  throw new PendingSolution();
}
