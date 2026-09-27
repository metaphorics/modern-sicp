// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.4: recall and and or from chapter 3: (and e1 ... en) returns
 * false as soon as one expression is false, otherwise the value of the
 * last one, with (and) true; (or e1 ... en) returns the first non-false
 * value and never evaluates the rest, with (or) false. Install and and or
 * as new special forms of the evaluator, and show that they short-circuit
 * and work in nested positions.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.4 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_04(): string {
  throw new PendingSolution();
}
