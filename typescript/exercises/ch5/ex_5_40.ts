// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 5.40: compile-time environment threading. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 5.40 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_5_40(): never {
  throw new PendingSolution();
}
