// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.15: sine by argument reduction. cube, p, and sine are
 * given by the statement; the pending artifact is the instrumented sine
 * that counts p's applications.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.15 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Sine by reduction, reporting how many times p was applied. */
export function sineWithCount(_angle: number): { value: number; pApplications: number } {
  throw new PendingSolution();
}
