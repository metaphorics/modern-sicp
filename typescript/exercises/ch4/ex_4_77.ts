// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 4_77: Delay negation until required variables are bound. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4_77 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_4_77(): string {
  throw new PendingSolution();
}
