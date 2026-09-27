// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.6a (added by this edition, extends exercise 2.6): arithmetic
 * on three plus the successor function, checked through the
 * encoded-decoding property churchToInt.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.6a is not solved yet");
    this.name = "PendingSolution";
  }
}

/** A Church numeral applies its step f, n times, to its seed x. */
export type Church = <T>(f: (x: T) => T) => (x: T) => T;

/** Three, defined directly. */
export const three: Church = (_f) => {
  throw new PendingSolution();
};

/** The successor function: the book's add1 under its arithmetic name. */
export function succ(_n: Church): Church {
  throw new PendingSolution();
}

/** Decodes a numeral: applies it to the successor step and a zero seed. */
export function churchToInt(n: Church): number {
  return n((x: number) => x + 1)(0);
}
