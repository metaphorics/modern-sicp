// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  div,
  makePolynomial,
  makeTsNumber,
  show,
} from "../../packages/ch2/src/05-generic-operations.js";

/**
 * Exercise 2.91: polynomial division. `div-terms` is the 2.91 blank
 * filled in --- long division on term lists, one generic `div` of
 * leading coefficients per step, recursing on the difference until
 * the divisor outranks what is left --- and `div-poly` checks the
 * indeterminate, delegates, and reattaches. The section system ships
 * both; this file pins the book's example.
 */

const makeX = (terms: ReadonlyArray<readonly [bigint, bigint]>) =>
  makePolynomial(
    "x",
    terms.map(([o, c]) => [o, makeTsNumber(c)] as const),
  );

/** The book's example: (x^5 - 1) / (x^2 - 1) = x^3 + x with remainder
 * x - 1, here read off the section system's div. */
export const bookDivision = (): string =>
  show(
    div(
      makeX([
        [5n, 1n],
        [0n, -1n],
      ]),
      makeX([
        [2n, 1n],
        [0n, -1n],
      ]),
    ),
  );
