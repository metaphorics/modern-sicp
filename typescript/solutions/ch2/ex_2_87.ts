// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import {
  coeffOf,
  firstTerm,
  type GenError,
  isEmptyTermListQ,
  isZeroQ,
  restTerms,
  type TermList,
} from "../../packages/ch2/src/05-generic-operations.js";

/**
 * Exercise 2.87: `=zero?` for polynomials. A polynomial is zero when
 * its term list is empty, and nonzero as soon as one coefficient is
 * not; the coefficients are tested through the generic `=zero?` of
 * exercise 2.80, so a coefficient that is itself a polynomial is
 * answered by the same dispatch --- the recursion the data-directed
 * style buys for free. The section system installs this entry when the
 * polynomial package is installed; this file spells the predicate the
 * entry runs.
 */

/** The polynomial zero-test entry: all coefficients zero, through the
 * generic predicate. */
export const isZeroTerms87 = (l: TermList): Result<boolean, GenError> => {
  if (isEmptyTermListQ(l)) {
    return ok(true);
  }
  const head = isZeroQ(coeffOf(firstTerm(l)));
  if (head._tag === "Error") {
    return head;
  }
  return head.value ? isZeroTerms87(restTerms(l)) : ok(false);
};
