// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 5.37: preserving disabled, compare stack waste. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 5.37 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_5_37(): never {
  throw new PendingSolution();
}
