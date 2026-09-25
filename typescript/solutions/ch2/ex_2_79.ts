// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { err, ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
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
 * Exercise 2.79: a generic `equ?`. Each package answers for its own
 * pairs of arguments --- the ordinary numbers by `===`, the rationals
 * by the n1 d2 = n2 d1 test of 2.1.1, the complex numbers by equal
 * parts --- and the generic operation dispatches on the argument tags.
 * The section's packages install these entries too; this file spells
 * the per-package predicates and the dispatch that ties them.
 */

/** The scheme-number equ? entry: primitive equality on exact
 * integers. */
export const equSchemeNumber = (x: bigint, y: bigint): boolean => x === y;

/** The rational equ? entry: the book's equal-rat?, cross-multiplied so
 * 1/2 equals 2/4. */
export const equRational = (x: RatContents, y: RatContents): boolean => x[0] * y[1] === y[0] * x[1];

/** The complex equ? entry: equal real parts and equal imaginary
 * parts, read one tag down through the 2.77 selectors. */
export const equComplex = (
  x: Rectangular | Polar,
  y: Rectangular | Polar,
): Result<boolean, GenError> => {
  const rx = repSelector("real-part")(x);
  const ry = repSelector("real-part")(y);
  const ix = repSelector("imag-part")(x);
  const iy = repSelector("imag-part")(y);
  return rx._tag === "Error"
    ? rx
    : ry._tag === "Error"
      ? ry
      : ix._tag === "Error"
        ? ix
        : iy._tag === "Error"
          ? iy
          : ok(rx.value === ry.value && ix.value === iy.value);
};

/** The rational contents behind an equ? argument: the only array shape
 * a rational entry can meet. */
const isRatContents = (c: ArithContents): c is RatContents =>
  Array.isArray(c) && typeof c[0] === "bigint";

/** The inner complex representation behind an equ? argument. */
const isInnerRep = (c: ArithContents): c is Rectangular | Polar => {
  if (Array.isArray(c) || typeof c !== "object") {
    return false;
  }
  if ("variable" in c || "numer" in c) {
    return false;
  }
  return "_tag" in c && (c._tag === "rectangular" || c._tag === "polar");
};

/** The generic equ?: dispatch on the argument tags to the package
 * entry. Arguments of different types have no entry, the book's "No
 * method for these types". */
export const equ79 = (x: ArithDatum, y: ArithDatum): Result<boolean, GenError> => {
  const tx = typeTagOf(x);
  const ty = typeTagOf(y);
  if (
    tx === "scheme-number" &&
    ty === "scheme-number" &&
    typeof x === "bigint" &&
    typeof y === "bigint"
  ) {
    return ok(equSchemeNumber(x, y));
  }
  if (tx === "rational" && ty === "rational") {
    const cx = contentsOf(x);
    const cy = contentsOf(y);
    return isRatContents(cx) && isRatContents(cy)
      ? ok(equRational(cx, cy))
      : err({ _tag: "NoMethod", op: "equ?", tags: [tx, ty] });
  }
  if (tx === "complex" && ty === "complex") {
    const cx = contentsOf(x);
    const cy = contentsOf(y);
    return isInnerRep(cx) && isInnerRep(cy)
      ? equComplex(cx, cy)
      : err({ _tag: "NoMethod", op: "equ?", tags: [tx, ty] });
  }
  return err({ _tag: "NoMethod", op: "equ?", tags: [tx, ty] });
};
