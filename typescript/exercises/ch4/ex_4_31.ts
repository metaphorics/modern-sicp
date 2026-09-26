// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.31: extend the evaluator with upward-compatible parameter
 * declarations: (name lazy) delays without memoization, (name lazy-memo)
 * delays with it, and ordinary definitions stay strict. Implement the new
 * syntax procedures and the application changes.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 4.31 is not solved yet");
    this.name = "PendingSolution";
  }
}

export function ex_4_31(): string {
  throw new PendingSolution();
}
