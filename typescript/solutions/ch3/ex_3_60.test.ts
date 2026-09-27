// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  addStreams,
  integersStartingFrom,
  streamMap2,
  streamTake,
} from "../../packages/ch3/src/05-streams.js";

import { cosineSeries, expSeries, mulSeries, sineSeries } from "./ex_3_60.js";

describe("exercise 3.60: mul-series convolution", () => {
  it("squares sine and cosine and the sum is the unit series", () => {
    const pythagorean = addStreams(
      mulSeries(sineSeries, sineSeries),
      mulSeries(cosineSeries, cosineSeries),
    );
    const coefficients = streamTake(pythagorean, 8);
    expect(coefficients[0]).toBe(1);
    for (let n = 1; n < 8; n += 1) {
      expect(coefficients[n]).toBeCloseTo(0, 12);
    }
  });

  it("multiplies exp(x) by itself into exp(2x)", () => {
    const expSquared = streamTake(mulSeries(expSeries, expSeries), 8);
    // e^(2x) = sum (2x)^n / n!, i.e. the exp coefficients scaled by 2^n.
    const exp2x = streamMap2((c, n) => c * 2 ** n, expSeries, integersStartingFrom(0));
    const expected = streamTake(exp2x, 8);
    expected.forEach((e, n) => {
      expect(expSquared[n]).toBeCloseTo(e, 12);
    });
  });
});
