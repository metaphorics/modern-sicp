// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import type { Result } from "../../packages/ch2/src/01-data-abstraction.js";

import {
  type ArithDatum,
  div,
  type GenError,
  greatestCommonDivisor,
  makeSchemeNumber,
  show,
} from "../../packages/ch2/src/05-generic-operations.js";
import { bookFirstRemainder, bookGcd, bookP1, bookP2 } from "./ex_2_94.js";

const unwrap = (r: Result<ArithDatum, GenError>): ArithDatum => {
  if (r._tag !== "Ok") {
    throw new Error("expected a polynomial");
  }
  return r.value;
};

describe("exercise 2.94: the polynomial gcd", () => {
  it("computes the book's example through the generic operation", () => {
    expect(bookGcd()).toBe("(polynomial x (2 -1) (1 1))");
  });

  it("remainder-terms answers the first Euclid remainder", () => {
    expect(bookFirstRemainder()).toBe("(polynomial x (2 -1) (1 1))");
  });

  it("the gcd divides both polys with zero remainder, by hand", () => {
    const g = unwrap(greatestCommonDivisor(bookP1, bookP2));
    expect(show(div(bookP1, g))).toBe(
      "(quotient-remainder (polynomial x (2 -1) (0 2)) (polynomial x ))",
    );
    expect(show(div(bookP2, g))).toBe(
      "(quotient-remainder (polynomial x (1 -1) (0 -1)) (polynomial x ))",
    );
  });

  it("reduces to ordinary gcd for ordinary numbers", () => {
    expect(show(greatestCommonDivisor(makeSchemeNumber(24n), makeSchemeNumber(36n)))).toBe("12");
  });
});
