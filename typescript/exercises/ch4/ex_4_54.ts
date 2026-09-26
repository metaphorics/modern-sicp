// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.54: complete analyze-require so require can be implemented as a special form instead of an ordinary procedure.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.54 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_54(): string {
  throw new PendingSolution();
}
