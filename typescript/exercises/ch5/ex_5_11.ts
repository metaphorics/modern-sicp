// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 5.11: the three save and restore disciplines. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 5.11 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_5_11(): never {
  throw new PendingSolution();
}
