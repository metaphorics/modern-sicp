// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { makeRatNormalized, printRat } from "./ex_2_01.js";

describe("exercise 2.1", () => {
  it("all four sign combinations of 1/2 normalize onto the numerator", () => {
    expect(printRat(makeRatNormalized(1n, 2n))).toBe("1/2");
    expect(printRat(makeRatNormalized(-1n, 2n))).toBe("-1/2");
    expect(printRat(makeRatNormalized(1n, -2n))).toBe("-1/2");
    expect(printRat(makeRatNormalized(-1n, -2n))).toBe("1/2");
  });

  it("the sign decision survives an unreduced negative denominator", () => {
    expect(printRat(makeRatNormalized(2n, -4n))).toBe("-1/2");
    expect(printRat(makeRatNormalized(-2n, -4n))).toBe("1/2");
    expect(printRat(makeRatNormalized(-6n, 4n))).toBe("-3/2");
  });
});
