// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.31: product as the accumulation of a range, factorial via
 * product, and pi/4 by John Wallis' formula
 * pi/4 = (2*4*4*6*6*8*...)/(3*3*5*5*7*7*...).
 * The iterative variant is a loop because Node gives no tail-call guarantee.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 1.31 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The product of term over a..b, in the section's recursive shape. */
export function product(
  _term: (x: number) => number,
  _a: number,
  _next: (x: number) => number,
  _b: number,
): number {
  throw new PendingSolution();
}

/** The same product as a while loop carrying the running result. */
export function productIter(
  _term: (x: number) => number,
  _a: number,
  _next: (x: number) => number,
  _b: number,
): number {
  throw new PendingSolution();
}

/** n! expressed as one call to product. */
export function factorial(_n: number): number {
  throw new PendingSolution();
}

/** pi/4 approximated by the Wallis product over the range 2..b stepping by 2. */
export function wallisPiQuarter(_b: number): number {
  throw new PendingSolution();
}
