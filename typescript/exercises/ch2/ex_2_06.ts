// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.6: Church numerals. The zero is itself a function - the
 * function that never applies its step - and one, two, and the addition
 * procedure must be defined directly, not through zero and add1.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.6 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** A Church numeral applies its step f, n times, to its seed x. */
export type Church = <T>(f: (x: T) => T) => (x: T) => T;

/** The statement's zero: the function that ignores its step. */
export const churchZero: Church = (_f) => {
  throw new PendingSolution();
};

/** The statement's add-1: applies the step one more time than n does. */
export function add1(_n: Church): Church {
  throw new PendingSolution();
}

/** One, defined directly. */
export const one: Church = (_f) => {
  throw new PendingSolution();
};

/** Two, defined directly. */
export const two: Church = (_f) => {
  throw new PendingSolution();
};

/** Addition, defined directly: not repeated application of add1. */
export function churchAdd(_m: Church, _n: Church): Church {
  throw new PendingSolution();
}

/** Decodes a numeral: applies it to the successor step and a zero seed. */
export function churchToInt(n: Church): number {
  return n((x: number) => x + 1)(0);
}
