// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.26: Louis Reasoner's expmod doubles the self-calls. The
 * pending artifact is the instrumented comparison of the two expmods'
 * call counts.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.26 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Self-call count of the square-based expmod for this exp. */
export function expmodSquareCalls(_exp: number): number {
  throw new PendingSolution();
}

/** Self-call count of Louis's explicit-multiplication expmod for this exp. */
export function expmodExplicitCalls(_exp: number): number {
  throw new PendingSolution();
}
