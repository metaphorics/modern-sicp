// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 0.2: implement `compose` and `repeated` over one-argument
 * procedures on numbers.
 *
 * `compose(f, g)(x)` applies `g` and then `f`; `repeated(f, n)(x)` applies
 * `f` exactly `n` times, with `repeated(f, 0)(x)` the identity. The
 * statement lives in the section 0.2 chapter text.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 0.2 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The one-argument procedure type both exercisers work over. */
export type Mapper = (x: number) => number;

export function compose(_f: Mapper, _g: Mapper): Mapper {
  throw new PendingSolution();
}

export function repeated(_f: Mapper, _n: number): Mapper {
  throw new PendingSolution();
}
