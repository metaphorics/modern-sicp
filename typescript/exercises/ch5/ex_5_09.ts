// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 5.09: refuse labels as operation operands. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 5.09 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_5_09(): never {
  throw new PendingSolution();
}
