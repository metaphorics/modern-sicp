// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 5.10: a new syntax behind isolated syntax procedures. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 5.10 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_5_10(): never {
  throw new PendingSolution();
}
