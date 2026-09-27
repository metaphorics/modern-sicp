// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 5.48: compile-and-run primitive inside evaluator. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 5.48 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_5_48(): never {
  throw new PendingSolution();
}
