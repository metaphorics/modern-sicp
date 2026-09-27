// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 4_79: Use local rule environments instead of variable renaming. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4_79 is not solved yet");
    this.name = "PendingSolution";
  }
}
export function ex_4_79(): string {
  throw new PendingSolution();
}
