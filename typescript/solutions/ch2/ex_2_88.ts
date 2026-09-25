// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { err, ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import { attachTag } from "../../packages/ch2/src/04-data-directed.js";
import type { Poly, TermList } from "../../packages/ch2/src/05-generic-operations.js";
import {
  type ArithContents,
  type ArithDatum,
  addTerms,
  applyGeneric,
  coeffOf,
  firstTerm,
  type GenError,
  isEmptyTermListQ,
  makeComplexFromRealImag,
  makePolynomial,
  makeRational,
  makeSchemeNumber,
  makeTerm,
  op,
  orderOf,
  put,
  restTerms,
  sameVariableQ,
  showArithDatum,
  theEmptyTermList,
  variableOf,
} from "../../packages/ch2/src/05-generic-operations.js";

/**
 * Exercise 2.88: negation and subtraction of polynomials. Negation is
 * installed as a package entry at every level this exercise covers
 * (ordinary numbers, rationals, complex numbers, polynomials), and
 * subtraction is addition of the negation --- the hint the statement
 * gives. The polynomial negate negates each coefficient through the
 * generic negate, so nested polynomial coefficients negate too.
 */

const mapTermList = (
  l: TermList,
  f: (c: ArithDatum) => Result<ArithDatum, GenError>,
): Result<TermList, GenError> => {
  if (isEmptyTermListQ(l)) {
    return ok(theEmptyTermList());
  }
  const t = firstTerm(l);
  const head = f(coeffOf(t));
  if (head._tag === "Error") {
    return head;
  }
  const rest = mapTermList(restTerms(l), f);
  return rest._tag === "Error" ? rest : ok([makeTerm(orderOf(t), head.value), ...rest.value]);
};

const negateDatum = (c: ArithDatum): Result<ArithDatum, GenError> => applyGeneric("negate", c);

/** Negates a poly, coefficient by coefficient through the generic
 * negate. */
export const negatePoly = (p: Poly): Result<Poly, GenError> => {
  const terms = mapTermList(p.terms, negateDatum);
  return terms._tag === "Error" ? terms : ok({ variable: p.variable, terms: terms.value });
};

const contentsPoly = (c: ArithContents): Poly | undefined =>
  typeof c === "object" && !Array.isArray(c) && "variable" in c && "terms" in c
    ? { variable: c.variable, terms: c.terms }
    : undefined;

const errNoNeg = (tag: string): Result<never, GenError> => ({
  _tag: "Error",
  error: { _tag: "NoMethod", op: "negate", tags: [tag] },
});

/** Reads the real coordinate of a complex contents. */
const complexReal = (c: ArithContents): number | undefined => {
  if (typeof c !== "object" || c === null || Array.isArray(c) || !("_tag" in c)) {
    return undefined;
  }
  if (c._tag === "rectangular") {
    return c.contents[0];
  }
  if (c._tag === "polar") {
    return c.contents[0] * Math.cos(c.contents[1]);
  }
  return undefined;
};

/** Reads the imaginary coordinate of a complex contents. */
const complexImag = (c: ArithContents): number | undefined => {
  if (typeof c !== "object" || c === null || Array.isArray(c) || !("_tag" in c)) {
    return undefined;
  }
  if (c._tag === "rectangular") {
    return c.contents[1];
  }
  if (c._tag === "polar") {
    return c.contents[0] * Math.sin(c.contents[1]);
  }
  return undefined;
};

/** Installs the negate entries of this exercise, and the polynomial
 * subtraction built on them. Idempotent. */
export const installNegation = (): void => {
  put(
    "negate",
    ["scheme-number"],
    op((args) => {
      const x = args[0];
      return typeof x === "bigint" ? ok(makeSchemeNumber(-x)) : errNoNeg("scheme-number");
    }),
  );
  put(
    "negate",
    ["rational"],
    op((args) => {
      const x = args[0];
      if (!Array.isArray(x) || typeof x[0] !== "bigint" || typeof x[1] !== "bigint") {
        return errNoNeg("rational");
      }
      return ok(makeRational(-x[0], x[1]));
    }),
  );
  put(
    "negate",
    ["complex"],
    op((args) => {
      const c = args[0];
      const re = c !== undefined ? complexReal(c) : undefined;
      const im = c !== undefined ? complexImag(c) : undefined;
      return re !== undefined && im !== undefined
        ? ok(makeComplexFromRealImag(-re, -im))
        : errNoNeg("complex");
    }),
  );
  put(
    "negate",
    ["polynomial"],
    op((args) => {
      const c = args[0];
      const p = c !== undefined ? contentsPoly(c) : undefined;
      if (p === undefined) {
        return errNoNeg("polynomial");
      }
      const negated = negatePoly(p);
      return negated._tag === "Error" ? negated : ok(attachTag("polynomial", negated.value));
    }),
  );
  put(
    "sub",
    ["polynomial", "polynomial"],
    op((args) => {
      const p1 = args[0] !== undefined ? contentsPoly(args[0]) : undefined;
      const p2 = args[1] !== undefined ? contentsPoly(args[1]) : undefined;
      if (p1 === undefined || p2 === undefined) {
        return err({ _tag: "NoMethod", op: "sub", tags: ["polynomial", "polynomial"] });
      }
      return subPoly(p1, p2);
    }),
  );
};

/** The polynomial subtraction of this exercise: add the negation. */
export const subPoly = (p1: Poly, p2: Poly): Result<ArithDatum, GenError> => {
  if (!sameVariableQ(variableOf(p1), variableOf(p2))) {
    const left = showArithDatum(attachTag("polynomial", p1));
    const right = showArithDatum(attachTag("polynomial", p2));
    return err({ _tag: "NotSameVar", proc: "SUB-POLY", left, right });
  }
  const negated = negatePoly(p2);
  if (negated._tag === "Error") {
    return negated;
  }
  const terms = addTerms(p1.terms, negated.value.terms);
  return terms._tag === "Error"
    ? terms
    : ok(attachTag("polynomial", { variable: p1.variable, terms: terms.value }));
};

/** The generic sub, now covering polynomials too. */
export const sub88 = (x: ArithDatum, y: ArithDatum): Result<ArithDatum, GenError> =>
  applyGeneric("sub", x, y);

/** The generic negation. */
export const negate = (x: ArithDatum): Result<ArithDatum, GenError> => applyGeneric("negate", x);

/** Builds a polynomial for the tests. */
export const poly88 = (v: string, terms: ReadonlyArray<readonly [bigint, bigint]>): ArithDatum =>
  makePolynomial(
    v,
    terms.map(([o, c]) => [o, makeSchemeNumber(c)] as const),
  );
