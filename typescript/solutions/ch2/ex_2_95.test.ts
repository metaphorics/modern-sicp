// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  makePolynomial,
  show,
  showArithDatum,
} from "../../packages/ch2/src/05-generic-operations.js";
import { firstDivisionStep, p1, p2, p3, q1, q2 } from "./ex_2_95.js";

describe("exercise 2.95: where integer arithmetic fails the gcd", () => {
  it("builds the book's products", () => {
    expect(show(q1())).toBe("(polynomial x (4 11) (3 -22) (2 18) (1 -14) (0 7))");
    expect(show(q2())).toBe("(polynomial x (3 13) (2 -21) (1 3) (0 5))");
  });

  it("the first division step already truncates to zero and cycles", () => {
    // 11/13 truncates to 0, so the quotient term contributes no terms
    // and the step subtracts nothing: gcd-terms would meet the same
    // (Q1, Q2) pair again, forever. remainder-terms is never called.
    const step = firstDivisionStep();
    expect(step._tag).toBe("Ok");
    if (step._tag !== "Ok") {
      return;
    }
    expect(showArithDatum(step.value.newC)).toBe("0");
    expect(step.value.product).toEqual([]);
    expect(showArithDatum(makePolynomial("x", step.value.dividend))).toBe(show(q1()));
  });

  it("pins the book's three polynomials", () => {
    expect(showArithDatum(p1)).toBe("(polynomial x (2 1) (1 -2) (0 1))");
    expect(showArithDatum(p2)).toBe("(polynomial x (2 11) (0 7))");
    expect(showArithDatum(p3)).toBe("(polynomial x (1 13) (0 5))");
  });
});
