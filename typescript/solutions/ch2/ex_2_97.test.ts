// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import type { Result } from "../../packages/ch2/src/01-data-abstraction.js";

import {
  type ArithDatum,
  type GenError,
  makePolynomial,
  makeTsNumber,
  mul,
  show,
} from "../../packages/ch2/src/05-generic-operations.js";
import { p1, p2, p3 } from "./ex_2_95.js";
import {
  addRatFn97,
  installReduce,
  makeRat97,
  poly97,
  reduce,
  reduceIntegers,
  reducePoly,
} from "./ex_2_97.js";

installReduce();

const unwrap = <A>(r: Result<A, GenError>): A => {
  if (r._tag !== "Ok") {
    throw new Error("expected an answer");
  }
  return r.value;
};

const unwrapPoly = (r: Result<ArithDatum, GenError>): ArithDatum => {
  if (r._tag !== "Ok") {
    throw new Error("expected a polynomial");
  }
  return r.value;
};

describe("exercise 2.97: reduce-terms and the generic reduce", () => {
  it("reduces integers the way the original makeRat did", () => {
    expect(reduceIntegers(24n, 36n)).toEqual([2n, 3n]);
  });

  it("reduce dispatches through applyGeneric for ts-number values", () => {
    expect(show(reduce(makeTsNumber(24n), makeTsNumber(36n)))).toBe("[rational, 2, 3]");
  });

  it("reduce dispatches through apply-generic for polynomials", () => {
    const q1 = unwrapPoly(mul(p1, p2));
    const q2 = unwrapPoly(mul(p1, p3));
    expect(show(reduce(q1, q2))).toBe(
      "[rational, [polynomial, x, [2, 11], [0, 7]], [polynomial, x, [1, 13], [0, 5]]]",
    );
  });

  it("reduce-poly refuses polys in different variables", () => {
    const x = poly97([[1n, 1n]]).contents;
    const y = makePolynomial("y", [[1n, makeTsNumber(1n)]]).contents;
    const r = reducePoly(x, y);
    expect(r._tag).toBe("Error");
    if (r._tag === "Error" && r.error._tag === "NotSameVar") {
      expect(r.error.proc).toBe("reducePoly");
    }
  });

  it("adds the book's rational functions to the correctly reduced answer", () => {
    // ((x+1)/(x^3-1)) + (x/(x^2-1)):
    // (x^4+x^3+x^2-2x-1) / (x^5-x^3-x^2+1)
    // = (x^3+2x^2+3x+1) / (x^4+x^3-x-1).
    const rf = unwrap(
      makeRat97(
        poly97([
          [1n, 1n],
          [0n, 1n],
        ]),
        poly97([
          [3n, 1n],
          [0n, -1n],
        ]),
      ),
    );
    const rf2 = unwrap(
      makeRat97(
        poly97([[1n, 1n]]),
        poly97([
          [2n, 1n],
          [0n, -1n],
        ]),
      ),
    );
    expect(show(addRatFn97(rf, rf2))).toBe(
      "[rational, [polynomial, x, [3, 1], [2, 2], [1, 3], [0, 1]], [polynomial, x, [4, 1], [3, 1], [1, -1], [0, -1]]]",
    );
  });
});
