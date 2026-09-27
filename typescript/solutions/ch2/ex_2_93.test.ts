// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { show, showArithDatum } from "../../packages/ch2/src/05-generic-operations.js";
import { addRatFn, makeRatFn, poly93 } from "./ex_2_93.js";

describe("exercise 2.93: rational functions", () => {
  it("builds a rational function of polynomials without reducing", () => {
    const rf = makeRatFn(
      poly93([
        [3n, 1n],
        [0n, 1n],
      ]),
      poly93([
        [2n, 1n],
        [0n, 1n],
      ]),
    );
    expect(showArithDatum(rf)).toBe(
      "(rational (polynomial x (3 1) (0 1)) (polynomial x (2 1) (0 1)))",
    );
  });

  it("adds rf to itself and does not reduce to lowest terms", () => {
    // The book's interaction: (x^3+1)/(x^2+1) added to itself leaves
    // 2(x^3+1)(x^2+1) over (x^2+1)^2, the common factor (x^2+1) intact.
    const rf = makeRatFn(
      poly93([
        [3n, 1n],
        [0n, 1n],
      ]),
      poly93([
        [2n, 1n],
        [0n, 1n],
      ]),
    );
    const sum = addRatFn(rf, rf);
    expect(sum._tag).toBe("Ok");
    expect(show(sum)).toBe(
      "(rational (polynomial x (5 2) (3 2) (2 2) (0 2)) (polynomial x (4 1) (2 2) (0 1)))",
    );
  });
});
