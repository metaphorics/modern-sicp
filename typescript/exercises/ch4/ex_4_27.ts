// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.27: with the lazy evaluator, define count = 0 and an id procedure
 * that increments count and returns its argument, then give the missing
 * values of the `w = id(id(10))` interaction sequence and explain
 * them.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.27 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_27(): string {
  throw new PendingSolution();
}
