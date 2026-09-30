// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import { attachTag } from "../../packages/ch2/src/04-data-directed.js";
import {
  type ArithDatum,
  applyGeneric,
  coeffOf,
  firstTerm,
  type GenError,
  isEmptyTermListQ,
  makeTerm,
  makeTsNumber,
  mulPoly,
  orderOf,
  type Poly,
  sameVariableQ,
  showArithDatum,
  type Term,
  type TermList,
} from "../../packages/ch2/src/05-generic-operations.js";

/**
 * Exercise 2.92: polynomials in different variables. The variables are
 * totally ordered (x above y above z here); an operation on two polys
 * in different variables promotes the lower-variable polynomial to the
 * higher variable as an order-zero coefficient, and the term merge
 * lifts bare number coefficients into the other coefficient's
 * variable, which is the number-to-polynomial coercion the section's
 * footnote calls for.
 */

/** The variable ordering, highest priority first: x outranks y
 * outranks z. */
const ORDER: ReadonlyArray<string> = ["x", "y", "z"];

const rankOf = (v: string): number => {
  const r = ORDER.indexOf(v);
  if (r < 0) {
    throw new Error(`unknown variable: ${v}`);
  }
  return ORDER.length - r;
};

const asPoly = (c: ArithDatum): Poly | undefined =>
  typeof c === "object" && !Array.isArray(c) && "_tag" in c && c._tag === "polynomial"
    ? c.contents
    : undefined;

const tagged = (p: Poly): ArithDatum => attachTag("polynomial", p);

const okTagged = (r: Result<Poly, GenError>): Result<ArithDatum, GenError> =>
  r._tag === "Error" ? r : ok(tagged(r.value));

const addPolyData = (a: Poly, b: Poly): Result<ArithDatum, GenError> => {
  const terms = mergeTerms(a.terms, b.terms, addCoeffs);
  return terms._tag === "Error" ? terms : ok(tagged({ variable: a.variable, terms: terms.value }));
};

/** Combines two coefficients by addition: bare with bare through the
 * generic add, poly with poly through addMulti, and a bare next to a
 * poly lifted into the poly's variable first. */
const addCoeffs = (c1: ArithDatum, c2: ArithDatum): Result<ArithDatum, GenError> => {
  const p1 = asPoly(c1);
  const p2 = asPoly(c2);
  if (p1 !== undefined && p2 !== undefined) {
    return p1.variable === p2.variable ? addPolyData(p1, p2) : addMulti(p1, p2);
  }
  if (p1 === undefined && p2 === undefined) {
    return applyGeneric("add", c1, c2);
  }
  const onlyPoly = p1 !== undefined ? p1 : (p2 as Poly);
  const bare = p1 !== undefined ? c2 : c1;
  const lifted: Poly = { variable: onlyPoly.variable, terms: [makeTerm(0n, bare)] };
  return addPolyData(lifted, onlyPoly);
};

/** Combines two coefficients by multiplication, with the same
 * promotions as the addition. */
const mulCoeffs = (c1: ArithDatum, c2: ArithDatum): Result<ArithDatum, GenError> => {
  const p1 = asPoly(c1);
  const p2 = asPoly(c2);
  if (p1 !== undefined && p2 !== undefined) {
    return p1.variable === p2.variable ? okTagged(mulPoly(p1, p2)) : mulMulti(p1, p2);
  }
  if (p1 === undefined && p2 === undefined) {
    return applyGeneric("mul", c1, c2);
  }
  const onlyPoly = p1 !== undefined ? p1 : (p2 as Poly);
  const bare = p1 !== undefined ? c2 : c1;
  const lifted: Poly = { variable: onlyPoly.variable, terms: [makeTerm(0n, bare)] };
  return okTagged(mulPoly(lifted, onlyPoly));
};

/** Multiplies one term by all terms of a list, coefficients combined
 * through mulCoeffs. */
const mulTermByAll92 = (t1: Term, l: TermList): Result<TermList, GenError> => {
  if (isEmptyTermListQ(l)) {
    return ok([]);
  }
  const t2 = firstTerm(l);
  const c = mulCoeffs(coeffOf(t1), coeffOf(t2));
  if (c._tag === "Error") {
    return c;
  }
  const rest = mulTermByAll92(t1, l.slice(1));
  if (rest._tag === "Error") {
    return rest;
  }
  return mergeTerms([makeTerm(orderOf(t1) + orderOf(t2), c.value)], rest.value, addCoeffs);
};

/** Multiplies two term lists of one variable, coefficients combined
 * through mulCoeffs and the products merged through addCoeffs. */
const mulTerms92 = (l1: TermList, l2: TermList): Result<TermList, GenError> => {
  if (isEmptyTermListQ(l1)) {
    return ok([]);
  }
  const product = mulTermByAll92(firstTerm(l1), l2);
  if (product._tag === "Error") {
    return product;
  }
  const rest = mulTerms92(l1.slice(1), l2);
  if (rest._tag === "Error") {
    return rest;
  }
  return mergeTerms(product.value, rest.value, addCoeffs);
};

const mergeTerms = (
  l1: TermList,
  l2: TermList,
  combine: (a: ArithDatum, b: ArithDatum) => Result<ArithDatum, GenError>,
): Result<TermList, GenError> => {
  if (isEmptyTermListQ(l1)) {
    return ok(l2);
  }
  if (isEmptyTermListQ(l2)) {
    return ok(l1);
  }
  const t1 = firstTerm(l1);
  const t2 = firstTerm(l2);
  if (orderOf(t1) > orderOf(t2)) {
    const rest = mergeTerms(l1.slice(1), l2, combine);
    return rest._tag === "Error" ? rest : ok([t1, ...rest.value]);
  }
  if (orderOf(t1) < orderOf(t2)) {
    const rest = mergeTerms(l1, l2.slice(1), combine);
    return rest._tag === "Error" ? rest : ok([t2, ...rest.value]);
  }
  const combined = combine(coeffOf(t1), coeffOf(t2));
  if (combined._tag === "Error") {
    return combined;
  }
  const rest = mergeTerms(l1.slice(1), l2.slice(1), combine);
  return rest._tag === "Error" ? rest : ok([makeTerm(orderOf(t1), combined.value), ...rest.value]);
};

/** Adds two polys, in the same or different variables. */
export const addMulti = (p1: Poly, p2: Poly): Result<ArithDatum, GenError> => {
  const sameVar = sameVariableQ(p1.variable, p2.variable);
  const higher = sameVar ? p1 : rankOf(p1.variable) >= rankOf(p2.variable) ? p1 : p2;
  const lower = sameVar ? p2 : rankOf(p1.variable) >= rankOf(p2.variable) ? p2 : p1;
  const augmented: Poly = sameVar
    ? lower
    : { variable: higher.variable, terms: [makeTerm(0n, tagged(lower))] };
  const terms = mergeTerms(higher.terms, augmented.terms, addCoeffs);
  return terms._tag === "Error"
    ? terms
    : ok(tagged({ variable: higher.variable, terms: terms.value }));
};

/** Multiplies two polys, in the same or different variables. */
export const mulMulti = (p1: Poly, p2: Poly): Result<ArithDatum, GenError> => {
  if (sameVariableQ(p1.variable, p2.variable)) {
    return okTagged(mulPoly(p1, p2));
  }
  const higher = rankOf(p1.variable) >= rankOf(p2.variable) ? p1 : p2;
  const lower = rankOf(p1.variable) >= rankOf(p2.variable) ? p2 : p1;
  const promoted: Poly = { variable: higher.variable, terms: [makeTerm(0n, tagged(lower))] };
  const terms = mulTerms92(higher.terms, promoted.terms);
  return terms._tag === "Error"
    ? terms
    : ok(tagged({ variable: higher.variable, terms: terms.value }));
};

/** Builds a poly from (order, coefficient-number) pairs. */
export const poly92 = (
  variable: string,
  terms: ReadonlyArray<readonly [bigint, bigint]>,
): Poly => ({
  variable,
  terms: terms.map(([o, c]) => makeTerm(o, makeTsNumber(c))),
});

/** Builds a poly from (order, datum) pairs, bare or polynomial
 * coefficients alike. */
export const polyMixed92 = (
  variable: string,
  terms: ReadonlyArray<readonly [bigint, ArithDatum]>,
): Poly => ({
  variable,
  terms: terms.map(([o, c]) => makeTerm(o, c)),
});

/** Builds a poly with polynomial coefficients. */
export const polyNested92 = (
  variable: string,
  terms: ReadonlyArray<readonly [bigint, Poly]>,
): Poly => ({
  variable,
  terms: terms.map(([o, c]) => makeTerm(o, tagged(c))),
});

/** Renders a poly the way the book prints it. */
export const show92 = (p: Poly): string => showArithDatum(tagged(p));
