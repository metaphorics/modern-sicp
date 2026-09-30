// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import {
  type ArithDatum,
  applyGeneric,
  coeffOf,
  divTerms,
  firstTerm,
  type GenError,
  isEmptyTermListQ,
  makeTerm,
  makeTsNumber,
  mulTerms,
  orderOf,
  remainderTerms,
  restTerms,
  type TermList,
  theEmptyTermList,
} from "../../packages/ch2/src/05-generic-operations.js";

/**
 * Exercise 2.96: pseudodivision. Before each division the dividend is
 * multiplied by the integerizing factor c^(1 + O1 - O2) --- c the
 * leading coefficient of the divisor --- so no fractions arise; the
 * remainder of that division is the pseudoremainder. The GCD built on
 * it then strips the content its steps accumulate by dividing every
 * coefficient by the (integer) greatest common divisor of the
 * coefficients.
 */

const bindL = <A, B>(
  r: Result<A, GenError>,
  f: (a: A) => Result<B, GenError>,
): Result<B, GenError> => (r._tag === "Ok" ? f(r.value) : r);

/** The integerizing factor c^(1 + O1 - O2) for a divisor's leading
 * coefficient c and the two degrees: through the system's exp, which
 * stays on exact integers. The reduction of 2.97 reuses this with O1
 * the maximum of the numerator's and denominator's degrees. */
export const integerizingFactorTo = (
  c: ArithDatum,
  o1: bigint,
  o2: bigint,
): Result<ArithDatum, GenError> => applyGeneric("exp", c, makeTsNumber(1n + o1 - o2));

const integerizingFactor = (p: TermList, q: TermList): Result<ArithDatum, GenError> =>
  integerizingFactorTo(coeffOf(firstTerm(q)), orderOf(firstTerm(p)), orderOf(firstTerm(q)));

/** The book's pseudoremainder-terms: scale the dividend by the
 * integerizing factor, then take the ordinary remainder. A dividend
 * the divisor already outranks is its own remainder --- the factor's
 * degree exponent would go negative, and no division can remove
 * anything. */
export const pseudoremainderTerms = (p: TermList, q: TermList): Result<TermList, GenError> =>
  orderOf(firstTerm(p)) < orderOf(firstTerm(q))
    ? ok(p)
    : bindL(integerizingFactor(p, q), (factor) =>
        bindL(mulTerms(p, [makeTerm(0n, factor)]), (scaled) => remainderTerms(scaled, q)),
      );

/** The same division answering the quotient too: the reduction of
 * 2.97 divides the scaled numerator and denominator by their GCD. */
export const pseudoDivTerms = (
  p: TermList,
  q: TermList,
): Result<readonly [TermList, TermList], GenError> =>
  bindL(integerizingFactor(p, q), (factor) =>
    bindL(mulTerms(p, [makeTerm(0n, factor)]), (scaled) => divTerms(scaled, q)),
  );

/** The integer greatest common divisor of a term list's coefficients,
 * folded through the generic greatest-common-divisor: the content the
 * pseudodivisions accumulate. An empty list has no content. */
export const contentOf = (l: TermList): Result<ArithDatum | undefined, GenError> => {
  const go = (
    rest: TermList,
    acc: ArithDatum | undefined,
  ): Result<ArithDatum | undefined, GenError> => {
    if (isEmptyTermListQ(rest)) {
      return ok(acc);
    }
    const head = coeffOf(firstTerm(rest));
    if (acc === undefined) {
      return go(restTerms(rest), head);
    }
    const step = applyGeneric("greatest-common-divisor", acc, head);
    return step._tag === "Error" ? step : go(restTerms(rest), step.value);
  };
  return go(l, undefined);
};

/** Divides every coefficient of a term list by the datum `g`: the
 * content-removal primitive the reduction of 2.97 reuses on the
 * numerator and denominator together. */
export const divideCoefficientsBy = (l: TermList, g: ArithDatum): Result<TermList, GenError> => {
  const go = (rest: TermList, acc: TermList): Result<TermList, GenError> => {
    if (isEmptyTermListQ(rest)) {
      return ok(acc);
    }
    const t = firstTerm(rest);
    const divided = applyGeneric("div", coeffOf(t), g);
    if (divided._tag === "Error") {
      return divided;
    }
    return go(restTerms(rest), [...acc, [orderOf(t), divided.value] as const]);
  };
  return go(l, theEmptyTermList());
};

/** Removes the content of a term list: every coefficient divided by
 * the greatest common divisor of the coefficients. */
export const removeContent = (l: TermList): Result<TermList, GenError> =>
  bindL(contentOf(l), (content) =>
    content === undefined ? ok(theEmptyTermList()) : divideCoefficientsBy(l, content),
  );

/** The gcd-terms of this exercise: Euclid over pseudoremainders, every
 * step's answer stripped of its content. */
export const gcdTerms96 = (a: TermList, b: TermList): Result<TermList, GenError> =>
  isEmptyTermListQ(b)
    ? ok(a)
    : bindL(pseudoremainderTerms(a, b), (r) => bindL(removeContent(r), (rc) => gcdTerms96(b, rc)));
