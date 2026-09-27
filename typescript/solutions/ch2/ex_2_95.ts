// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import {
  type ArithDatum,
  coeffOf,
  div,
  firstTerm,
  type GenError,
  makePolynomial,
  makeSchemeNumber,
  makeTerm,
  mul,
  mulTerms,
  orderOf,
  type TermList,
} from "../../packages/ch2/src/05-generic-operations.js";

/**
 * Exercise 2.95: where integer arithmetic fails the polynomial GCD.
 * The book's P1, P2, P3 and the products Q1 = P1*P2, Q2 = P1*P3 share
 * the factor P1, but the Euclid run of gcd-terms never sees it: the
 * very first division step divides the leading coefficients 11 by 13,
 * which the edition's truncating integer division answers with 0. The
 * quotient term contributes no terms, the step subtracts nothing, and
 * the algorithm would hand the same pair back to itself forever. The
 * run is therefore examined exactly one step deep, replicated through
 * firstDivisionStep; remainder-terms is never called on Q1 and Q2.
 */

const bindL = <A, B>(
  r: Result<A, GenError>,
  f: (a: A) => Result<B, GenError>,
): Result<B, GenError> => (r._tag === "Ok" ? f(r.value) : r);

const poly95 = (terms: ReadonlyArray<readonly [bigint, bigint]>) =>
  makePolynomial(
    "x",
    terms.map(([o, c]) => [o, makeSchemeNumber(c)] as const),
  );

/** The book's P1: x^2 - 2x + 1. */
export const p1 = poly95([
  [2n, 1n],
  [1n, -2n],
  [0n, 1n],
]);

/** The book's P2: 11x^2 + 7. */
export const p2 = poly95([
  [2n, 11n],
  [0n, 7n],
]);

/** The book's P3: 13x + 5. */
export const p3 = poly95([
  [1n, 13n],
  [0n, 5n],
]);

/** Q1, the product of P1 and P2. */
export const q1 = (): Result<ArithDatum, GenError> => mul(p1, p2);

/** Q2, the product of P1 and P3. */
export const q2 = (): Result<ArithDatum, GenError> => mul(p1, p3);

const termsOf = (r: Result<ArithDatum, GenError>): Result<TermList, GenError> => {
  if (r._tag === "Error") {
    return r;
  }
  if (typeof r.value === "bigint") {
    return ok([]);
  }
  return r.value._tag === "polynomial" ? ok(r.value.contents.terms) : ok([]);
};

/** The first division step of gcd-terms(Q1, Q2), replicated without
 * calling remainder-terms. newC is the truncated 11/13; product is the
 * divisor times the quotient term; dividend is what the next step
 * would receive --- the other half of the pair it started with. */
export const firstDivisionStep = (): Result<
  { readonly newC: ArithDatum; readonly product: TermList; readonly dividend: TermList },
  GenError
> =>
  bindL(termsOf(q1()), (dividend) =>
    bindL(termsOf(q2()), (divisor) => {
      const newC = div(coeffOf(firstTerm(dividend)), coeffOf(firstTerm(divisor)));
      if (newC._tag === "Error") {
        return newC;
      }
      const newO = orderOf(firstTerm(dividend)) - orderOf(firstTerm(divisor));
      return bindL(mulTerms(divisor, [makeTerm(newO, newC.value)]), (product) =>
        ok({ newC: newC.value, product, dividend }),
      );
    }),
  );
