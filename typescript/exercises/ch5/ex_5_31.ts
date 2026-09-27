// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 5.31: which evaluator saves are superfluous. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 5.31 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_5_31(): never {
  throw new PendingSolution();
}
