// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.12: Pascal's triangle. The pending artifact is the
 * procedure computing elements of the triangle by a recursive process.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.12 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The element of Pascal's triangle at (row, col), 0-indexed. */
export function pascal(_row: number, _col: number): number {
  throw new PendingSolution();
}
