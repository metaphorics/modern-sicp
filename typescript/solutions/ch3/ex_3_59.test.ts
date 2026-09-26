// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { ones, streamTake } from "../../packages/ch3/src/05-streams.js";

import { cosineSeries, expSeries, integrateSeries, sineSeries } from "./ex_3_59.js";

describe("exercise 3.59: integrate-series and the elementary series", () => {
  it("integrates the ones stream into a0/n: 1, 1/2, 1/3, 1/4, 1/5", () => {
    const coefficients = streamTake(integrateSeries(ones), 5);
    expect(coefficients[0]).toBe(1);
    expect(coefficients[1]).toBeCloseTo(1 / 2, 12);
    expect(coefficients[2]).toBeCloseTo(1 / 3, 12);
    expect(coefficients[3]).toBeCloseTo(1 / 4, 12);
    expect(coefficients[4]).toBeCloseTo(1 / 5, 12);
  });

  it("builds exp-series: 1 + x + x^2/2 + x^3/6 + x^4/24 + x^5/120", () => {
    const coefficients = streamTake(expSeries, 6);
    expect(coefficients[0]).toBe(1);
    expect(coefficients[1]).toBe(1);
    expect(coefficients[2]).toBeCloseTo(1 / 2, 12);
    expect(coefficients[3]).toBeCloseTo(1 / 6, 12);
    expect(coefficients[4]).toBeCloseTo(1 / 24, 12);
    expect(coefficients[5]).toBeCloseTo(1 / 120, 12);
  });

  it("builds cosine-series: 1 + 0x - x^2/2 + 0x^3 + x^4/24 + 0x^5 - x^6/720", () => {
    const coefficients = streamTake(cosineSeries, 7);
    expect(coefficients[0]).toBe(1);
    expect(coefficients[1]).toBe(0);
    expect(coefficients[2]).toBeCloseTo(-1 / 2, 12);
    expect(coefficients[3]).toBe(0);
    expect(coefficients[4]).toBeCloseTo(1 / 24, 12);
    expect(coefficients[5]).toBe(0);
    expect(coefficients[6]).toBeCloseTo(-1 / 720, 12);
  });

  it("builds sine-series: 0 + x + 0x^2 - x^3/6 + 0x^4 + x^5/120 + 0x^6 - x^7/5040", () => {
    const coefficients = streamTake(sineSeries, 8);
    expect(coefficients[0]).toBe(0);
    expect(coefficients[1]).toBe(1);
    expect(coefficients[2]).toBe(0);
    expect(coefficients[3]).toBeCloseTo(-1 / 6, 12);
    expect(coefficients[4]).toBe(0);
    expect(coefficients[5]).toBeCloseTo(1 / 120, 12);
    expect(coefficients[6]).toBe(0);
    expect(coefficients[7]).toBeCloseTo(-1 / 5040, 12);
  });
});
