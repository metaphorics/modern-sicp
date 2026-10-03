// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.52: implement failure fallback, which catches the failure of its first expression and answers with its second expression instead.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.52 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_52(): string {
  throw new PendingSolution();
}
