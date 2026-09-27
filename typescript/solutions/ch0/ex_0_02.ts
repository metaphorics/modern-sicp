// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 0.2: `compose` and `repeated` over one-argument number
 * procedures.
 *
 * `compose(f, g)` is a closure that applies `g` first; `repeated` recurses
 * on `n - 1`, with the `n = 0` case the identity. The recursion is on the
 * count, not on the values, so it runs at any practical depth.
 */
export type Mapper = (x: number) => number;

/** Applies `g` and then `f`. */
export const compose =
  (f: Mapper, g: Mapper): Mapper =>
  (x) =>
    f(g(x));

/** Applies `f` exactly n times; zero times is the identity. */
export const repeated =
  (f: Mapper, n: number): Mapper =>
  (x) =>
    n === 0 ? x : f(repeated(f, n - 1)(x));
