// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 5.41: find-variable returns lexical address. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 5.41 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_5_41(): never {
  throw new PendingSolution();
}
