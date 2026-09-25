// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { err, ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import { attachTag } from "../../packages/ch2/src/04-data-directed.js";
import {
  type ArithContents,
  type ArithDatum,
  applyGeneric,
  coeffOf,
  divTerms,
  firstTerm,
  type GenError,
  gcdInteger,
  makePoly,
  makePolynomial,
  makeSchemeNumber,
  makeTerm,
  mulTerms,
  op,
  orderOf,
  type Poly,
  type Polynomial,
  put,
  type RationalFunction,
  sameVariableQ,
  showArithDatum,
  type TermList,
  termListOf,
  typeTagOf,
  variableOf,
} from "../../packages/ch2/src/05-generic-operations.js";
import { contentOf, divideCoefficientsBy, gcdTerms96, integerizingFactorTo } from "./ex_2_96.js";

/**
 * Exercise 2.97: reducing rational functions to lowest terms.
 * reduce-terms computes the GCD of 2.96, scales numerator and
 * denominator by the same integerizing factor --- the GCD's leading
 * coefficient to the 1 + O1 - O2 of the larger degree --- divides both
 * by the GCD, and finally divides every coefficient of both by the
 * greatest common divisor of all the coefficients together.
 * reduce-poly is reduce-terms behind the same-variable check, the
 * integer version is the original make-rat's gcd, and the generic
 * reduce dispatches to either, answering the reduced pair under the
 * section's rational tag, numer and denom.
 */

const bindL = <A, B>(
  r: Result<A, GenError>,
  f: (a: A) => Result<B, GenError>,
): Result<B, GenError> => (r._tag === "Ok" ? f(r.value) : r);

const combinedContent = (nn: TermList, dd: TermList): Result<ArithDatum | undefined, GenError> =>
  bindL(contentOf(nn), (cn) =>
    bindL(contentOf(dd), (cd) => {
      if (cn === undefined) {
        return ok(cd);
      }
      if (cd === undefined) {
        return ok(cn);
      }
      const g = applyGeneric("greatest-common-divisor", cn, cd);
      return g._tag === "Error" ? g : ok(g.value);
    }),
  );

/** The book's reduce-terms: two term lists in, their lowest terms out,
 * numerator first. */
export const reduceTerms = (
  n: TermList,
  d: TermList,
): Result<readonly [TermList, TermList], GenError> => {
  const oN = orderOf(firstTerm(n));
  const oD = orderOf(firstTerm(d));
  const o1 = oN > oD ? oN : oD;
  const gStep = gcdTerms96(n, d);
  if (gStep._tag === "Error") {
    return gStep;
  }
  const g = gStep.value;
  const fStep = integerizingFactorTo(coeffOf(firstTerm(g)), o1, orderOf(firstTerm(g)));
  if (fStep._tag === "Error") {
    return fStep;
  }
  const zero = makeTerm(0n, fStep.value);
  const sN = mulTerms(n, [zero]);
  if (sN._tag === "Error") {
    return sN;
  }
  const sD = mulTerms(d, [zero]);
  if (sD._tag === "Error") {
    return sD;
  }
  const nStep = divTerms(sN.value, g);
  if (nStep._tag === "Error") {
    return nStep;
  }
  const dStep = divTerms(sD.value, g);
  if (dStep._tag === "Error") {
    return dStep;
  }
  const cStep = combinedContent(nStep.value[0], dStep.value[0]);
  if (cStep._tag === "Error") {
    return cStep;
  }
  if (cStep.value === undefined) {
    return ok([nStep.value[0], dStep.value[0]] as const);
  }
  const nn = divideCoefficientsBy(nStep.value[0], cStep.value);
  if (nn._tag === "Error") {
    return nn;
  }
  const dd = divideCoefficientsBy(dStep.value[0], cStep.value);
  if (dd._tag === "Error") {
    return dd;
  }
  return ok([nn.value, dd.value] as const);
};

/** The book's reduce-poly, on the model of add-poly: the two polys
 * must share their indeterminate; the reduced term lists carry it
 * again. */
export const reducePoly = (p1: Poly, p2: Poly): Result<readonly [Poly, Poly], GenError> => {
  if (!sameVariableQ(variableOf(p1), variableOf(p2))) {
    return err({
      _tag: "NotSameVar",
      proc: "REDUCE-POLY",
      left: showArithDatum(attachTag("polynomial", p1)),
      right: showArithDatum(attachTag("polynomial", p2)),
    });
  }
  return bindL(reduceTerms(termListOf(p1), termListOf(p2)), ([nn, dd]) =>
    ok([makePoly(variableOf(p1), nn), makePoly(variableOf(p1), dd)] as const),
  );
};

/** The book's reduce-integers: what the original make-rat did. */
export const reduceIntegers = (n: bigint, d: bigint): readonly [bigint, bigint] => {
  const g = gcdInteger(n, d);
  return [n / g, d / g];
};

const polyArg97 = (c: ArithContents | undefined): Poly | undefined =>
  typeof c === "object" && !Array.isArray(c) && "variable" in c && "terms" in c ? c : undefined;

const miss97 = (tags: ReadonlyArray<string>): Result<never, GenError> =>
  err({ _tag: "NoMethod", op: "reduce", tags });

const isRatFn = (d: ArithDatum): d is RationalFunction =>
  typeof d !== "bigint" &&
  d._tag === "rational" &&
  !Array.isArray(d.contents) &&
  "numer" in d.contents;

/** Installs the reduce entries of this exercise: integers through the
 * gcd of 1.2.5, polynomials through reduce-poly, both answering the
 * reduced pair under the rational tag. Idempotent. */
export const installReduce = (): void => {
  put(
    "reduce",
    ["scheme-number", "scheme-number"],
    op((args) => {
      const n = args[0];
      const d = args[1];
      if (typeof n !== "bigint" || typeof d !== "bigint") {
        return miss97(["scheme-number", "scheme-number"]);
      }
      const [nn, dd] = reduceIntegers(n, d);
      return ok(attachTag("rational", { numer: nn, denom: dd }));
    }),
  );
  put(
    "reduce",
    ["polynomial", "polynomial"],
    op((args) => {
      const p1 = polyArg97(args[0]);
      const p2 = polyArg97(args[1]);
      if (p1 === undefined || p2 === undefined) {
        return miss97(["polynomial", "polynomial"]);
      }
      return bindL(reducePoly(p1, p2), ([nn, dd]) => {
        const numer: ArithDatum = attachTag("polynomial", nn);
        const denom: ArithDatum = attachTag("polynomial", dd);
        return ok(attachTag("rational", { numer, denom }));
      });
    }),
  );
};

/** The book's generic reduce: apply-generic dispatches to reduce-poly
 * for polynomial arguments and to reduce-integers for scheme numbers. */
export const reduce = (n: ArithDatum, d: ArithDatum): Result<RationalFunction, GenError> => {
  const r = applyGeneric("reduce", n, d);
  if (r._tag === "Error") {
    return r;
  }
  return isRatFn(r.value) ? ok(r.value) : miss97([typeTagOf(n), typeTagOf(d)]);
};

/** The book's make-rat after 2.97: reduce before combining, so a
 * rational function is stored in lowest terms from birth. */
export const makeRat97 = (n: ArithDatum, d: ArithDatum): Result<RationalFunction, GenError> =>
  reduce(n, d);

/** Builds a polynomial datum with scheme-number coefficients. */
export const poly97 = (terms: ReadonlyArray<readonly [bigint, bigint]>): Polynomial =>
  makePolynomial(
    "x",
    terms.map(([o, c]) => [o, makeSchemeNumber(c)] as const),
  );

/** Adds two rational functions, constructing through reduce: the sum
 * lands in lowest terms. */
export const addRatFn97 = (
  x: RationalFunction,
  y: RationalFunction,
): Result<RationalFunction, GenError> => {
  const n1 = applyGeneric("mul", x.contents.numer, y.contents.denom);
  if (n1._tag === "Error") {
    return n1;
  }
  const n2 = applyGeneric("mul", y.contents.numer, x.contents.denom);
  if (n2._tag === "Error") {
    return n2;
  }
  const num = applyGeneric("add", n1.value, n2.value);
  if (num._tag === "Error") {
    return num;
  }
  const den = applyGeneric("mul", x.contents.denom, y.contents.denom);
  if (den._tag === "Error") {
    return den;
  }
  return makeRat97(num.value, den.value);
};
