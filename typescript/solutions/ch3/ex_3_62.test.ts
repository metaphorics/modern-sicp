// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { streamTake } from "../../packages/ch3/src/05-streams.js";

import { cosineSeries, divSeries, mulSeries, sineSeries, tangent } from "./ex_3_62.js";

describe("exercise 3.62: div-series and the tangent series", () => {
  it("refuses a denominator whose constant term is 0", () => {
    expect(() => divSeries(cosineSeries, sineSeries)).toThrow(Error);
    expect(() => divSeries(cosineSeries, sineSeries)).toThrow(
      "div-series: the denominator has a zero constant term",
    );
  });

  it("builds tangent = sin/cos: 0 + x + 0x^2 + x^3/3 + 0x^4 + 2x^5/15 + 0x^6 + 17x^7/315", () => {
    const coefficients = streamTake(tangent, 8);
    expect(coefficients[0]).toBe(0);
    expect(coefficients[1]).toBeCloseTo(1, 12);
    expect(coefficients[2]).toBeCloseTo(0, 12);
    expect(coefficients[3]).toBeCloseTo(1 / 3, 12);
    expect(coefficients[4]).toBeCloseTo(0, 12);
    expect(coefficients[5]).toBeCloseTo(2 / 15, 12);
    expect(coefficients[6]).toBeCloseTo(0, 12);
    expect(coefficients[7]).toBeCloseTo(17 / 315, 12);
  });

  it("divides any two series with a nonzero denominator constant: sin*cos over cos recovers sin", () => {
    const quotient = streamTake(divSeries(mulSeries(sineSeries, cosineSeries), cosineSeries), 8);
    const expected = streamTake(sineSeries, 8);
    expected.forEach((e, n) => {
      expect(quotient[n]).toBeCloseTo(e, 12);
    });
  });
});
