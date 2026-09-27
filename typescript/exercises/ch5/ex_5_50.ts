// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 5.50: compile the metacircular evaluator. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 5.50 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_5_50(): never {
  throw new PendingSolution();
}
