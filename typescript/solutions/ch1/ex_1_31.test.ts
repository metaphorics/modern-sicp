// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { factorial, product, productIter, wallisPiQuarter } from "./ex_1_31.js";

describe("exercise 1.31", () => {
  it("multiplies the range in both shapes", () => {
    expect(
      product(
        (x) => x,
        1,
        (x) => x + 1,
        10,
      ),
    ).toBe(3628800);
    expect(
      productIter(
        (x) => x,
        1,
        (x) => x + 1,
        10,
      ),
    ).toBe(3628800);
    expect(
      product(
        (x) => x * x,
        1,
        (x) => x + 1,
        5,
      ),
    ).toBe(14400);
  });

  it("the two shapes agree over a grid of ranges", () => {
    // Ranges short enough that every partial product stays an exact
    // integer below 2^53, where the two fold orders cannot disagree.
    const inc = (x: number): number => x + 1;
    for (let a = 1; a <= 6; a += 1) {
      for (let b = a; b <= a + 10; b += 1) {
        expect(product((x) => x + 1, a, inc, b)).toBe(productIter((x) => x + 1, a, inc, b));
      }
    }
  });

  it("factorial falls out of product", () => {
    expect(factorial(5)).toBe(120);
    expect(factorial(10)).toBe(3628800);
  });

  it("the Wallis product approaches pi/4", () => {
    expect(4 * wallisPiQuarter(10000)).toBeCloseTo(Math.PI, 3);
    expect(4 * wallisPiQuarter(100000)).toBe(3.141608361278168);
    expect(Math.abs(4 * wallisPiQuarter(100000) - Math.PI)).toBeLessThan(1e-4);
  });
});
