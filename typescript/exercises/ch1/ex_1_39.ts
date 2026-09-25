// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.39: Lambert's continued fraction
 * tan x = x/(1 - x^2/(3 - x^2/(5 - ...))), x in radians, computed with k
 * terms.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.39 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** tan x approximated by the k-term Lambert continued fraction. */
export function tanCf(_x: number, _k: number): number {
  throw new PendingSolution();
}
