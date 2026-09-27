// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 5.19: breakpoints with proceed and cancel. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 5.19 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_5_19(): never {
  throw new PendingSolution();
}
