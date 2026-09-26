// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 4.58: define big-shot as a person with no supervisor in the same division. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.58 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_4_58(): string {
  throw new PendingSolution();
}
