// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import {
  type ArithContents,
  type ArithDatum,
  contentsOf,
  type GenError,
  type Polar,
  type RatContents,
  type Rectangular,
  repSelector,
  typeTagOf,
} from "../../packages/ch2/src/05-generic-operations.js";

/**
 * Exercise 2.80: a generic `=zero?`. Each package answers for its own
 * arguments --- the ordinary numbers against `0`, the rationals by
 * numerator, the complex numbers by both parts --- and the generic
 * operation dispatches on the tag. Exercise 2.87 extends the same
 * operation to polynomials, which is what lets `adjoin-term` skip zero
 * polynomial coefficients.
 */

/** The scheme-number =zero? entry. */
export const isZeroSchemeNumber = (x: bigint): boolean => x === 0n;

/** The rational =zero? entry: the numerator is zero. */
export const isZeroRational = (x: RatContents): boolean => x[0] === 0n;

/** The complex =zero? entry: real and imaginary parts both zero, read
 * one tag down through the 2.77 selectors. */
export const isZeroComplex = (x: Rectangular | Polar): Result<boolean, GenError> => {
  const r = repSelector("real-part")(x);
  const i = repSelector("imag-part")(x);
  return r._tag === "Error" ? r : i._tag === "Error" ? i : ok(r.value === 0 && i.value === 0);
};

/** The rational contents behind a =zero? argument: the only array
 * shape a rational entry can meet. */
const isRatContents = (c: ArithContents): c is RatContents =>
  Array.isArray(c) && typeof c[0] === "bigint";

/** The inner complex representation behind a =zero? argument. */
const isInnerRep = (c: ArithContents): c is Rectangular | Polar => {
  if (Array.isArray(c) || typeof c !== "object") {
    return false;
  }
  if ("variable" in c || "numer" in c) {
    return false;
  }
  return "_tag" in c && (c._tag === "rectangular" || c._tag === "polar");
};

/** The generic =zero?: dispatch on the tag to the package entry. */
export const isZero80 = (x: ArithDatum): Result<boolean, GenError> => {
  const tag = typeTagOf(x);
  if (tag === "scheme-number" && typeof x === "bigint") {
    return ok(isZeroSchemeNumber(x));
  }
  if (tag === "rational") {
    const c = contentsOf(x);
    return isRatContents(c) ? ok(isZeroRational(c)) : err80(tag);
  }
  if (tag === "complex") {
    const c = contentsOf(x);
    return isInnerRep(c) ? isZeroComplex(c) : err80(tag);
  }
  return err80(tag);
};

const err80 = (tag: string): Result<never, GenError> => ({
  _tag: "Error",
  error: { _tag: "NoMethod", op: "=zero?", tags: [tag] },
});
