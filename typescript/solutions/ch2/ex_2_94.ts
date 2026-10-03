// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  greatestCommonDivisor,
  makePoly,
  makePolynomial,
  makeTsNumber,
  remainderTerms,
  show,
  showError,
  showPoly,
} from "../../packages/ch2/src/05-generic-operations.js";

/**
 * Exercise 2.94: the polynomial GCD. The section system ships the
 * exercise whole: remainder-terms picks the remainder out of div-terms,
 * gcd-terms is Euclid over those remainders, gcd-poly checks the
 * indeterminate, and the generic greatest-common-divisor reduces to
 * gcd-poly for polynomials and to ordinary gcd for ordinary numbers.
 * This file is the book's example, run through the generic entry; the
 * hand check --- the answer divides both polys with zero remainder ---
 * is pinned in the test.
 */

const poly94 = (terms: ReadonlyArray<readonly [bigint, bigint]>) =>
  makePolynomial(
    "x",
    terms.map(([o, c]) => [o, makeTsNumber(c)] as const),
  );

/** The book's p1: x^4 - x^3 - 2x^2 + 2x. */
export const bookP1 = poly94([
  [4n, 1n],
  [3n, -1n],
  [2n, -2n],
  [1n, 2n],
]);

/** The book's p2: x^3 - x. */
export const bookP2 = poly94([
  [3n, 1n],
  [1n, -1n],
]);

/** The book's test interaction: the greatest common divisor of the two
 * polys, through the generic operation. */
export const bookGcd = (): string => show(greatestCommonDivisor(bookP1, bookP2));

/** The first Euclid remainder of the example: remainder-terms applied
 * to the book's two term lists. */
export const bookFirstRemainder = (): string => {
  const r = remainderTerms(bookP1.contents.terms, bookP2.contents.terms);
  return r._tag === "Ok" ? showPoly(makePoly("x", r.value)) : showError(r.error);
};
