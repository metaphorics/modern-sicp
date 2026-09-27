// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.41: double applies a procedure twice, and
 * double(double(double))(inc)(5) is the puzzle of the statement.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.41 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The procedure that applies f twice. */
export function double<T>(_f: (x: T) => T): (x: T) => T {
  throw new PendingSolution();
}

/** The value returned by double(double(double))(inc)(5) with inc adding 1. */
export function puzzleAnswer(): number {
  throw new PendingSolution();
}
