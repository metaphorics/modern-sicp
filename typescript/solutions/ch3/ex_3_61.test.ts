// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { streamTake } from "../../packages/ch3/src/05-streams.js";

import { expSeries, invertUnitSeries, mulSeries } from "./ex_3_61.js";

describe("exercise 3.61: invert-unit-series", () => {
  it("inverts exp-series into e^-x: 1, -1, 1/2, -1/6, 1/24, -1/120", () => {
    const coefficients = streamTake(invertUnitSeries(expSeries), 6);
    expect(coefficients[0]).toBe(1);
    expect(coefficients[1]).toBe(-1);
    expect(coefficients[2]).toBeCloseTo(1 / 2, 12);
    expect(coefficients[3]).toBeCloseTo(-1 / 6, 12);
    expect(coefficients[4]).toBeCloseTo(1 / 24, 12);
    expect(coefficients[5]).toBeCloseTo(-1 / 120, 12);
  });

  it("multiplies exp-series by its inverse into the unit series", () => {
    const product = streamTake(mulSeries(expSeries, invertUnitSeries(expSeries)), 6);
    expect(product[0]).toBe(1);
    for (let n = 1; n < 6; n += 1) {
      expect(product[n]).toBeCloseTo(0, 12);
    }
  });
});
