// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { err, ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import { attachTag, type Tagged } from "../../packages/ch2/src/04-data-directed.js";
import {
  type ArithDatum,
  applyGeneric,
  coeffOf,
  firstTerm,
  type GenError,
  isEmptyTermListQ,
  isZeroQ,
  makeTerm,
  orderOf,
  restTerms,
  showPoly,
  type Term,
  type TermList,
  theEmptyTermList,
} from "../../packages/ch2/src/05-generic-operations.js";
import {
  adjoinTermDense,
  type DenseList,
  firstTermDense,
  isEmptyDense,
  restTermsDense,
  theEmptyDense,
} from "./ex_2_89.js";

/**
 * Exercise 2.90: sparse and dense term lists in one system. The two
 * representations are two values of one term-list interface --- the
 * selectors, constructors, and predicates of the section --- and a
 * polynomial tags which representation its term list is in. Adding
 * dispatches through the representation's interface; a mixed addition
 * refuses, the book's "Polys not in same var" shape.
 */

/** One term-list representation: everything add-terms and mul-terms
 * need, at the section's interface. */
export interface TermListOps<L> {
  readonly theEmpty: () => L;
  readonly isEmpty: (l: L) => boolean;
  readonly first: (l: L) => Term;
  readonly rest: (l: L) => L;
  readonly adjoin: (t: Term, l: L) => Result<L, GenError>;
}

/** The sparse package: the section's own list of `[order, coeff]` pairs. */
export const sparseOps: TermListOps<TermList> = {
  theEmpty: theEmptyTermList,
  isEmpty: isEmptyTermListQ,
  first: firstTerm,
  rest: restTerms,
  adjoin: (t, l) => ok(adjoinSparse(t, l)),
};

const adjoinSparse = (t: Term, l: TermList): TermList => {
  const zero = isZeroQ(coeffOf(t));
  return zero._tag === "Ok" && zero.value ? l : [t, ...l];
};

/** The dense package: the exercise 2.89 coefficients-only list. */
export const denseOps: TermListOps<DenseList> = {
  theEmpty: theEmptyDense,
  isEmpty: isEmptyDense,
  first: firstTermDense,
  rest: restTermsDense,
  adjoin: (t, l) => adjoinTermDense(t, l),
};

/** A polynomial tagged with its representation. */
export type Poly90 =
  | Tagged<
      "polynomial",
      { readonly rep: "sparse"; readonly variable: string; readonly terms: TermList }
    >
  | Tagged<
      "polynomial",
      { readonly rep: "dense"; readonly variable: string; readonly terms: DenseList }
    >;

/** Builds a sparse polynomial. */
export const makePolySparse = (variable: string, terms: TermList): Poly90 =>
  attachTag("polynomial", { rep: "sparse", variable, terms });

/** Builds a dense polynomial from (order, coefficient) pairs. */
export const makePolyDense = (
  variable: string,
  terms: ReadonlyArray<readonly [bigint, ArithDatum]>,
): Result<Poly90, GenError> => {
  let l: DenseList = theEmptyDense();
  for (const [o, c] of terms) {
    const adjoined = adjoinTermDense(makeTerm(o, c), l);
    if (adjoined._tag === "Error") {
      return adjoined;
    }
    l = adjoined.value;
  }
  return ok(attachTag("polynomial", { rep: "dense", variable, terms: l }));
};

/** Collects the nonzero terms of a term list, through one
 * representation's interface. */
const collectTerms = <L>(l0: L, ops: TermListOps<L>): Term[] => {
  const kept: Term[] = [];
  let l = l0;
  while (!ops.isEmpty(l)) {
    const t = ops.first(l);
    const zero = isZeroQ(coeffOf(t));
    if (!(zero._tag === "Ok" && zero.value)) {
      kept.push(t);
    }
    l = ops.rest(l);
  }
  return kept;
};

/** The variable and printed form both representations share: the
 * sparse spelling of the term list, zeros dropped. */
export const showPoly90 = (p: Poly90): string => {
  if (p.contents.rep === "sparse") {
    return showPoly({
      variable: p.contents.variable,
      terms: collectTerms(p.contents.terms, sparseOps),
    });
  }
  return showPoly({
    variable: p.contents.variable,
    terms: collectTerms(p.contents.terms, denseOps),
  });
};

const sameVar = (p1: Poly90, p2: Poly90): boolean =>
  p1.contents.rep === p2.contents.rep && p1.contents.variable === p2.contents.variable;

/** Adds two polynomials of the same representation through that
 * representation's interface. */
export const addPoly90 = (p1: Poly90, p2: Poly90): Result<Poly90, GenError> => {
  if (!sameVar(p1, p2)) {
    return err({
      _tag: "NotSameVar",
      proc: "addPoly90",
      left: showPoly90(p1),
      right: showPoly90(p2),
    });
  }
  if (p1.contents.rep === "sparse" && p2.contents.rep === "sparse") {
    const sum = addTermsGeneric(p1.contents.terms, p2.contents.terms, sparseOps);
    return sum._tag === "Error"
      ? sum
      : ok(
          attachTag("polynomial", {
            rep: "sparse",
            variable: p1.contents.variable,
            terms: sum.value,
          }),
        );
  }
  if (p1.contents.rep === "dense" && p2.contents.rep === "dense") {
    const sum = addTermsGeneric(p1.contents.terms, p2.contents.terms, denseOps);
    return sum._tag === "Error"
      ? sum
      : ok(
          attachTag("polynomial", {
            rep: "dense",
            variable: p1.contents.variable,
            terms: sum.value,
          }),
        );
  }
  return err({
    _tag: "NotSameVar",
    proc: "addPoly90",
    left: showPoly90(p1),
    right: showPoly90(p2),
  });
};

const addTermsGeneric = <L>(l1: L, l2: L, ops: TermListOps<L>): Result<L, GenError> => {
  if (ops.isEmpty(l1)) {
    return ok(l2);
  }
  if (ops.isEmpty(l2)) {
    return ok(l1);
  }
  const t1 = ops.first(l1);
  const t2 = ops.first(l2);
  if (orderOf(t1) > orderOf(t2)) {
    const rest = addTermsGeneric(ops.rest(l1), l2, ops);
    return rest._tag === "Error" ? rest : ops.adjoin(t1, rest.value);
  }
  if (orderOf(t1) < orderOf(t2)) {
    const rest = addTermsGeneric(l1, ops.rest(l2), ops);
    return rest._tag === "Error" ? rest : ops.adjoin(t2, rest.value);
  }
  const sum = applyGeneric("add", coeffOf(t1), coeffOf(t2));
  if (sum._tag === "Error") {
    return sum;
  }
  const rest = addTermsGeneric(ops.rest(l1), ops.rest(l2), ops);
  return rest._tag === "Error" ? rest : ops.adjoin(makeTerm(orderOf(t1), sum.value), rest.value);
};

/** Builds the book's sparse B = x^100 + 2x^2 + 1: three terms. */
export const sparseB = (): Poly90 =>
  makePolySparse("x", [makeTerm(100n, 1n), makeTerm(2n, 2n), makeTerm(0n, 1n)]);

/** Builds the same polynomial in the dense representation: a hundred
 * and one coefficients, most of them zero. */
export const denseB = (): Result<Poly90, GenError> =>
  makePolyDense("x", [
    [100n, 1n],
    [2n, 2n],
    [0n, 1n],
  ]);
