// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.42: compose returns the function x |-> f(g(x)).
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.42 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The composition f after g. */
export function compose(
  _f: (x: number) => number,
  _g: (x: number) => number,
): (x: number) => number {
  throw new PendingSolution();
}
