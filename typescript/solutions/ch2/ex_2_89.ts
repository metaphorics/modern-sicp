// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import {
  type ArithDatum,
  applyGeneric,
  coeffOf,
  type GenError,
  isZeroQ,
  makeTerm,
  orderOf,
  showPoly,
  type Term,
} from "../../packages/ch2/src/05-generic-operations.js";

/**
 * Exercise 2.89: the dense term-list representation. A dense
 * polynomial stores only the coefficients, highest order first, so
 * the book's A = x^5 + 2x^4 + 3x^2 - 2x - 5 is (1 2 0 3 -2 -5). The
 * order of a term is the length of the list from its coefficient to
 * the end, decremented by one; the selectors and constructor below
 * keep the same (order, coefficient) interface the sparse procedures
 * are written against.
 */

/** A dense term list: the coefficients alone, highest order first. */
export type DenseList = ReadonlyArray<ArithDatum>;

/** The book's the-empty-termlist, dense. */
export const theEmptyDense = (): DenseList => [];

/** The book's empty-termlist?, dense. */
export const isEmptyDense = (l: DenseList): boolean => l.length === 0;

/** The book's first-term, dense: the order is read off the length. */
export const firstTermDense = (l: DenseList): Term => {
  const coeff = l[0];
  if (coeff === undefined) {
    throw new Error("first-term: empty dense term list");
  }
  return [BigInt(l.length - 1), coeff];
};

/** The book's rest-terms, dense: dropping the head drops every order
 * by one, which the length-derived order absorbs. */
export const restTermsDense = (l: DenseList): DenseList => l.slice(1);

/** The book's adjoin-term, dense: a zero coefficient changes nothing;
 * otherwise the list is extended, padded with zeros, or overwritten
 * so the coefficient lands at exactly the term's order. */
export const adjoinTermDense = (t: Term, l: DenseList): Result<DenseList, GenError> => {
  const zero = isZeroQ(coeffOf(t));
  if (zero._tag === "Error") {
    return zero;
  }
  if (zero.value) {
    return ok(l);
  }
  const order = Number(orderOf(t));
  if (order < l.length) {
    const index = l.length - 1 - order;
    return ok([...l.slice(0, index), coeffOf(t), ...l.slice(index + 1)]);
  }
  const pad: ArithDatum[] = [];
  for (let i = l.length; i < order; i++) {
    pad.push(0n);
  }
  return ok([coeffOf(t), ...pad, ...l]);
};

/** Adds two dense term lists by the ordered merge of add-terms, on
 * the dense selectors. */
export const addTermsDense = (l1: DenseList, l2: DenseList): Result<DenseList, GenError> => {
  if (isEmptyDense(l1)) {
    return ok(l2);
  }
  if (isEmptyDense(l2)) {
    return ok(l1);
  }
  const t1 = firstTermDense(l1);
  const t2 = firstTermDense(l2);
  if (orderOf(t1) > orderOf(t2)) {
    const rest = addTermsDense(restTermsDense(l1), l2);
    return rest._tag === "Error" ? rest : adjoinTermDense(t1, rest.value);
  }
  if (orderOf(t1) < orderOf(t2)) {
    const rest = addTermsDense(l1, restTermsDense(l2));
    return rest._tag === "Error" ? rest : adjoinTermDense(t2, rest.value);
  }
  const sum = applyGeneric("add", coeffOf(t1), coeffOf(t2));
  if (sum._tag === "Error") {
    return sum;
  }
  const rest = addTermsDense(restTermsDense(l1), restTermsDense(l2));
  return rest._tag === "Error"
    ? rest
    : adjoinTermDense(makeTerm(orderOf(t1), sum.value), rest.value);
};

/** Builds a dense term list from (order, coefficient) pairs. */
export const denseOf = (terms: ReadonlyArray<Term>): Result<DenseList, GenError> => {
  let l: DenseList = theEmptyDense();
  for (const t of terms) {
    const adjoined = adjoinTermDense(t, l);
    if (adjoined._tag === "Error") {
      return adjoined;
    }
    l = adjoined.value;
  }
  return ok(l);
};

/** Renders a dense list in the sparse term-list shape the section
 * prints, so the two representations are comparable. */
export const showDense = (variable: string, l: DenseList): string => {
  const terms: Term[] = [];
  for (let i = 0; i < l.length; i++) {
    const c = l[i];
    if (c !== undefined) {
      terms.push(makeTerm(BigInt(l.length - 1 - i), c));
    }
  }
  const kept: Term[] = [];
  for (const t of terms) {
    const zero = isZeroQ(coeffOf(t));
    if (zero._tag === "Ok" && !zero.value) {
      kept.push(t);
    }
  }
  return showPoly({ variable, terms: kept });
};
