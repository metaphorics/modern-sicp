// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.16: an iterative exponentiation process using successive
 * squaring and a logarithmic number of steps, with the invariant
 * a * b^n unchanged from state to state.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.16 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** b^n by successive squaring, keeping the invariant a * b^n. */
export function fastExptIter(_b: number, _n: number): number {
  throw new PendingSolution();
}

/** The sequence of (a, b, n) states, to watch the invariant hold. */
export function fastExptIterStates(
  _b: number,
  _n: number,
): Array<{ a: number; b: number; n: number }> {
  throw new PendingSolution();
}
