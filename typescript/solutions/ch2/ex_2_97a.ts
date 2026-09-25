// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { err, ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import {
  type ArithDatum,
  type GenError,
  makePolynomial,
  mul,
  orderOf,
  showPoly,
  type TermList,
} from "../../packages/ch2/src/05-generic-operations.js";
import { p1, p2, p3 } from "./ex_2_95.js";
import { gcdTerms96 } from "./ex_2_96.js";
import { reduceTerms } from "./ex_2_97.js";

/**
 * Exercise 2.97a (added by this edition, extends exercise 2.97; the
 * map's "drop gcd-terms after reduce" check): a reduction is complete
 * only when nothing is left to drop. Running gcd-terms on a reduced
 * numerator and denominator must answer a unit --- a degree-zero
 * constant --- since any further common factor would contradict the
 * reduction; reducing a reduced pair must answer that same pair; and
 * the exercise 2.95 products Q1 = P1*P2 over Q2 = P1*P3 must reduce to
 * exactly P2 over P3, the shared P1 dropped whole.
 */

const bindL = <A, B>(
  r: Result<A, GenError>,
  f: (a: A) => Result<B, GenError>,
): Result<B, GenError> => (r._tag === "Ok" ? f(r.value) : r);

/** The gcd-terms of a reduced pair: the quantity the check requires to
 * be a unit. */
export const gcdAfterReduce = (n: TermList, d: TermList): Result<TermList, GenError> =>
  bindL(reduceTerms(n, d), ([nn, dd]) => gcdTerms96(nn, dd));

/** Whether a term list is a unit: one term, degree zero. */
export const isUnitTermList = (l: TermList): boolean => {
  const t = l[0];
  return l.length === 1 && t !== undefined && orderOf(t) === 0n;
};

/** Reduces the pair twice: once directly, once starting from the
 * already reduced answer. The round trip is the idempotence pin. */
export const reduceRoundTrip = (
  n: TermList,
  d: TermList,
): Result<
  {
    readonly once: readonly [TermList, TermList];
    readonly twice: readonly [TermList, TermList];
  },
  GenError
> =>
  bindL(reduceTerms(n, d), (once) =>
    bindL(reduceTerms(once[0], once[1]), (twice) => ok({ once, twice })),
  );

const termsOfDatum = (r: Result<ArithDatum, GenError>): Result<TermList, GenError> => {
  if (r._tag === "Error") {
    return r;
  }
  if (typeof r.value === "bigint") {
    return err({ _tag: "NoMethod", op: "terms", tags: ["scheme-number"] });
  }
  return r.value._tag === "polynomial"
    ? ok(r.value.contents.terms)
    : err({ _tag: "NoMethod", op: "terms", tags: [r.value._tag] });
};

/** The exercise 2.95 instance: Q1 over Q2, reduced. The numer and
 * denom must be P2 and P3 exactly. */
export const reducedInstance = (): Result<
  { readonly numer: TermList; readonly denom: TermList },
  GenError
> =>
  bindL(termsOfDatum(mul(p1, p2)), (q1Terms) =>
    bindL(termsOfDatum(mul(p1, p3)), (q2Terms) =>
      bindL(reduceTerms(q1Terms, q2Terms), ([nn, dd]) => ok({ numer: nn, denom: dd })),
    ),
  );

/** Renders a term list the way the book prints the polynomial it
 * belongs to. */
export const showTerms = (l: TermList): string => showPoly(makePolynomial("x", l).contents);
