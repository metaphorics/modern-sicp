// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import { attachTag, type Tagged } from "../../packages/ch2/src/04-data-directed.js";
import {
  type ArithDatum,
  applyGeneric,
  contentsOf,
  type GenError,
  makeRational,
  typeTagOf,
} from "../../packages/ch2/src/05-generic-operations.js";

/**
 * Exercise 2.86: complex numbers whose parts, magnitudes, and angles
 * are themselves tower datums. Once the parts are generic, every
 * primitive the rectangular and polar procedures use --- here square
 * root, sine, cosine, and sum --- must itself be generic. This file
 * builds the rectangular layer over generic parts and a numeric
 * abstraction for the operations the trigonometry needs.
 */

/** A rectangular complex number with tower datums as coordinates. */
export type Complex86 = Tagged<
  "rectangular86",
  { readonly real: ArithDatum; readonly imag: ArithDatum }
>;

/** Builds a generic-parts complex number from generic parts. */
export const makeComplex86 = (real: ArithDatum, imag: ArithDatum): Complex86 =>
  attachTag("rectangular86", { real, imag });

/** The real coordinate, already a datum. */
export const realPart86 = (z: Complex86): ArithDatum => z.contents.real;

/** The imaginary coordinate. */
export const imagPart86 = (z: Complex86): ArithDatum => z.contents.imag;

const miss86 = (opName: string, tag: string): Result<never, GenError> => ({
  _tag: "Error",
  error: { _tag: "NoMethod", op: opName, tags: [tag] },
});

const asNumber = (d: ArithDatum): number | undefined => {
  const c = contentsOf(d);
  if (typeof c === "bigint") {
    return Number(c);
  }
  if (typeof c === "number") {
    return c;
  }
  if (Array.isArray(c) && typeof c[0] === "bigint" && typeof c[1] === "bigint") {
    return Number(c[0]) / Number(c[1]);
  }
  return undefined;
};

/** The generic sine: exact types answer an inexact real, which is the
 * honest shape of a transcendental over the rationals. */
export const sine = (d: ArithDatum): Result<ArithDatum, GenError> => {
  const n = asNumber(d);
  return n === undefined ? miss86("sine", typeTagOf(d)) : ok(attachTag("real", Math.sin(n)));
};

/** The generic cosine. */
export const cosine = (d: ArithDatum): Result<ArithDatum, GenError> => {
  const n = asNumber(d);
  return n === undefined ? miss86("cosine", typeTagOf(d)) : ok(attachTag("real", Math.cos(n)));
};

/** The integer square root of a nonnegative bigint, by Newton's
 * method: the exactness test the generic sqrt needs. */
export const isqrt = (n: bigint): bigint => {
  if (n < 2n) {
    return n;
  }
  let x = n;
  let y = (x + 1n) / 2n;
  while (y < x) {
    x = y;
    y = (x + n / x) / 2n;
  }
  return x;
};

const isPerfectSquare = (n: bigint): boolean => isqrt(n) * isqrt(n) === n;

/** The generic square root: a perfect-square integer stays an exact
 * integer, a rational with perfect-square parts stays exact, and
 * anything else falls to an inexact real. */
export const sqrt = (d: ArithDatum): Result<ArithDatum, GenError> => {
  const c = contentsOf(d);
  if (typeof c === "bigint") {
    return c >= 0n && isPerfectSquare(c)
      ? ok(isqrt(c))
      : ok(attachTag("real", Math.sqrt(Number(c))));
  }
  if (Array.isArray(c) && typeof c[0] === "bigint" && typeof c[1] === "bigint" && c[0] >= 0n) {
    return isPerfectSquare(c[0]) && isPerfectSquare(c[1])
      ? ok(makeRational(isqrt(c[0]), isqrt(c[1])))
      : ok(attachTag("real", Math.sqrt(Number(c[0]) / Number(c[1]))));
  }
  if (typeof c === "number" && c >= 0) {
    return ok(attachTag("real", Math.sqrt(c)));
  }
  return miss86("sqrt", typeTagOf(d));
};

/** The generic sum, over the section's table. */
const sum = (a: ArithDatum, b: ArithDatum): Result<ArithDatum, GenError> =>
  applyGeneric("add", a, b);

/** Squares a datum through the generic mul. */
const square = (d: ArithDatum): Result<ArithDatum, GenError> => applyGeneric("mul", d, d);

/** The magnitude of a generic-parts complex number: the Pythagorean
 * sum runs entirely through the generic operations, so an exact
 * answer comes out exactly when the arithmetic allows it. */
export const magnitude86 = (z: Complex86): Result<ArithDatum, GenError> => {
  const sq1 = square(realPart86(z));
  if (sq1._tag === "Error") {
    return sq1;
  }
  const sq2 = square(imagPart86(z));
  if (sq2._tag === "Error") {
    return sq2;
  }
  const total = sum(sq1.value, sq2.value);
  if (total._tag === "Error") {
    return total;
  }
  return sqrt(total.value);
};

/** The sum of two generic-parts complex numbers. */
export const addComplex86 = (z1: Complex86, z2: Complex86): Result<Complex86, GenError> => {
  const re = sum(realPart86(z1), realPart86(z2));
  if (re._tag === "Error") {
    return re;
  }
  const im = sum(imagPart86(z1), imagPart86(z2));
  if (im._tag === "Error") {
    return im;
  }
  return ok(makeComplex86(re.value, im.value));
};

/** The angle of a generic-parts complex number, generic atan2 over
 * the inexact reals. */
export const angle86 = (z: Complex86): Result<ArithDatum, GenError> => {
  const re = asNumber(realPart86(z));
  const im = asNumber(imagPart86(z));
  return re === undefined || im === undefined
    ? miss86("atan2", "rectangular86")
    : ok(attachTag("real", Math.atan2(im, re)));
};
