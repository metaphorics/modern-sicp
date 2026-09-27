// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 4_75: Implement a unique query form. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4_75 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_4_75(): string {
  throw new PendingSolution();
}
