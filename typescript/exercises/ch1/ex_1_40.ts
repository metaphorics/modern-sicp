// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.40: cubic returns the function x |-> x^3 + ax^2 + bx + c for
 * use as newtonsMethod(cubic(a, b, c), guess).
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.40 is not solved yet");
    this.name = "PendingSolution";
  }
}
/** The cubic x^3 + ax^2 + bx + c as a one-argument function. */
export function cubic(_a: number, _b: number, _c: number): (x: number) => number {
  throw new PendingSolution();
}
