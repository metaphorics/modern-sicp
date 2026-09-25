// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 2.4: the alternative procedural representation of pairs. A
 * pair is the function pair type: a function that hands both of its parts
 * to a selector, and the selector decides the result's type. The
 * statement's cons and car, plus the corresponding cdr; the substitution
 * verification of the pair law is carPair(consPair(x, y)) =
 * consPair(x, y)((p) => p) = ((p) => p)(x, y) = x.
 */
export type Pair<A, B> = <R>(m: (x: A, y: B) => R) => R;

/** The procedural cons: no data structure, just the selector application. */
export const consPair =
  <A, B>(x: A, y: B): Pair<A, B> =>
  (m) =>
    m(x, y);

/** Applies the pair to the selector that keeps the first part. */
export const carPair = <A, B>(z: Pair<A, B>): A => z((p) => p);

/** The corresponding definition: the selector that keeps the second part. */
export const cdrPair = <A, B>(z: Pair<A, B>): B => z((_p, q) => q);
