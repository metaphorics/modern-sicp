// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** Exercise 2.25: pick out the 7 from three nested combinations. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.25 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The 7 inside (1 3 (5 7) 9). */
export function sevenFromFirst(): number {
  throw new PendingSolution();
}

/** The 7 inside ((7)). */
export function sevenFromSecond(): number {
  throw new PendingSolution();
}

/** The 7 inside (1 (2 (3 (4 (5 (6 7)))))). */
export function sevenFromThird(): number {
  throw new PendingSolution();
}
