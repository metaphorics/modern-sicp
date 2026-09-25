// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.5: Ben Bitdiddle's test for applicative-order versus
 * normal-order evaluation. The statement's two procedures are given;
 * the pending part is the answer: what Ben observes under each order,
 * and why.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.5 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Diverges: a body that only calls itself again. */
export const p = (): never => p();

/** Returns 0 when x is 0; the argument expressions are evaluated first. */
export const test = (x: number, y: number): number => (x === 0 ? 0 : y);

export function ex_1_05(): string {
  throw new PendingSolution();
}
