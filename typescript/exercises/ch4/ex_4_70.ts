// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 4_70: Explain the assertion-stream let binding. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4_70 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_4_70(): string {
  throw new PendingSolution();
}
