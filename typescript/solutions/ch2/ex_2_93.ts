// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import { attachTag, type Tagged } from "../../packages/ch2/src/04-data-directed.js";
import {
  type ArithDatum,
  add,
  type GenError,
  makePolynomial,
  makeSchemeNumber,
  mul,
  sub,
} from "../../packages/ch2/src/05-generic-operations.js";

/**
 * Exercise 2.93: rational functions. The rational-number package is
 * rebuilt over the generic operations, with `make-rat` reduced to a
 * plain pair --- no gcd, no lowest terms. The parts are tower datums,
 * so a rational function of polynomials is built exactly like the
 * rational numbers of 2.1.1; the unreduced sums are the point the
 * extended exercise turns on.
 */

/** A rational function: two tower datums, unreduced. */
export type RatFn = Tagged<"rational", { readonly numer: ArithDatum; readonly denom: ArithDatum }>;

/** The book's make-rat after this exercise: pair the parts, reduce
 * nothing. */
export const makeRatFn = (numer: ArithDatum, denom: ArithDatum): RatFn =>
  attachTag("rational", { numer, denom });

/** The book's numer and denom. */
export const numerOf = (x: RatFn): ArithDatum => x.contents.numer;
export const denomOf = (x: RatFn): ArithDatum => x.contents.denom;

const bind2 = (
  a: Result<ArithDatum, GenError>,
  b: Result<ArithDatum, GenError>,
  f: (x: ArithDatum, y: ArithDatum) => Result<ArithDatum, GenError>,
): Result<ArithDatum, GenError> =>
  a._tag === "Error" ? a : b._tag === "Error" ? b : f(a.value, b.value);

const ratFn2 = (
  nn: Result<ArithDatum, GenError>,
  dd: Result<ArithDatum, GenError>,
): Result<ArithDatum, GenError> =>
  nn._tag === "Error" ? nn : dd._tag === "Error" ? dd : ok(makeRatFn(nn.value, dd.value));

/** Adds: (n1 d2 + n2 d1) over (d1 d2), generic in the parts. */
export const addRatFn = (x: RatFn, y: RatFn): Result<ArithDatum, GenError> =>
  ratFn2(
    bind2(mul(numerOf(x), denomOf(y)), mul(numerOf(y), denomOf(x)), add),
    mul(denomOf(x), denomOf(y)),
  );

/** Subtracts: (n1 d2 - n2 d1) over (d1 d2). */
export const subRatFn = (x: RatFn, y: RatFn): Result<ArithDatum, GenError> =>
  ratFn2(
    bind2(mul(numerOf(x), denomOf(y)), mul(numerOf(y), denomOf(x)), sub),
    mul(denomOf(x), denomOf(y)),
  );

/** Multiplies: (n1 n2) over (d1 d2). */
export const mulRatFn = (x: RatFn, y: RatFn): Result<ArithDatum, GenError> =>
  ratFn2(mul(numerOf(x), numerOf(y)), mul(denomOf(x), denomOf(y)));

/** Divides: (n1 d2) over (d1 n2). */
export const divRatFn = (x: RatFn, y: RatFn): Result<ArithDatum, GenError> =>
  ratFn2(mul(numerOf(x), denomOf(y)), mul(denomOf(x), numerOf(y)));

/** Builds a polynomial datum. */
export const poly93 = (terms: ReadonlyArray<readonly [bigint, bigint]>): ArithDatum =>
  makePolynomial(
    "x",
    terms.map(([o, c]) => [o, makeSchemeNumber(c)] as const),
  );
