// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import type { Result } from "../../packages/ch2/src/01-data-abstraction.js";

import {
  type ArithDatum,
  div,
  type GenError,
  makePolynomial,
  makeSchemeNumber,
  show,
  showPoly,
  type TermList,
} from "../../packages/ch2/src/05-generic-operations.js";
import { p1, q1, q2 } from "./ex_2_95.js";
import {
  gcdTerms96,
  integerizingFactorTo,
  pseudoremainderTerms,
  removeContent,
} from "./ex_2_96.js";

const termsOf = (r: Result<ArithDatum, GenError>): TermList => {
  if (r._tag !== "Ok" || typeof r.value === "bigint" || r.value._tag !== "polynomial") {
    throw new Error("expected a polynomial");
  }
  return r.value.contents.terms;
};

const showTerms = (l: TermList): string => showPoly(makePolynomial("x", l).contents);

describe("exercise 2.96: pseudodivision", () => {
  it("scales by the integerizing factor c^(1 + O1 - O2)", () => {
    // 13^(1 + 4 - 3) for the book's Q1 over Q2.
    expect(show(integerizingFactorTo(makeSchemeNumber(13n), 4n, 3n))).toBe("169");
  });

  it("computes the pseudoremainder of Q1 and Q2", () => {
    const pr = pseudoremainderTerms(termsOf(q1()), termsOf(q2()));
    expect(pr._tag).toBe("Ok");
    expect(showTerms(pr._tag === "Ok" ? pr.value : [])).toBe(
      "(polynomial x (2 1458) (1 -2916) (0 1458))",
    );
  });

  it("removes the content the pseudodivision accumulated", () => {
    const pr = pseudoremainderTerms(termsOf(q1()), termsOf(q2()));
    expect(pr._tag).toBe("Ok");
    const rc = removeContent(pr._tag === "Ok" ? pr.value : []);
    expect(showTerms(rc._tag === "Ok" ? rc.value : [])).toBe("(polynomial x (2 1) (1 -2) (0 1))");
  });

  it("gcd-terms now answers P1 with integer coefficients", () => {
    const g = gcdTerms96(termsOf(q1()), termsOf(q2()));
    expect(showTerms(g._tag === "Ok" ? g.value : [])).toBe("(polynomial x (2 1) (1 -2) (0 1))");
  });

  it("the answer divides both products exactly, by hand", () => {
    // Q1 / P1 = P2 and Q2 / P1 = P3 with zero remainders.
    const uq1 = termsOf(q1());
    const uq2 = termsOf(q2());
    expect(show(div(makePolynomial("x", uq1), p1))).toBe(
      "(quotient-remainder (polynomial x (2 11) (0 7)) (polynomial x ))",
    );
    expect(show(div(makePolynomial("x", uq2), p1))).toBe(
      "(quotient-remainder (polynomial x (1 13) (0 5)) (polynomial x ))",
    );
  });
});
