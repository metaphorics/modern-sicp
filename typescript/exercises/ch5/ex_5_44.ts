// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 5.44: open-coding respects shadowing names. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 5.44 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_5_44(): never {
  throw new PendingSolution();
}
