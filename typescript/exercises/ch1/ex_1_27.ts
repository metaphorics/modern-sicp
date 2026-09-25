// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.27: demonstrate that the Carmichael numbers of footnote 47
 * fool the Fermat test. The pending artifact is the exhaustive
 * congruence check.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.27 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** True when a^n is congruent to a mod n for every 1 <= a < n. */
export function passesFermatForAllA(_n: number): boolean {
  throw new PendingSolution();
}
