// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { streamRef, streamTake } from "../../packages/ch3/src/05-streams.js";

import { estimateIntegral, unitCircleEstimates } from "./ex_3_82.js";

describe("exercise 3.82: Monte Carlo integration as streams", () => {
  it("estimates the unit circle's area, coarse first and refining", () => {
    expect(streamTake(unitCircleEstimates, 8)).toEqual([
      0, 0, 1.3333333333333333, 2, 2.4, 2.6666666666666665, 2.2857142857142856, 2.5,
    ]);
  });

  it("has estimate 200 within a quarter unit of pi", () => {
    const estimate = streamRef(unitCircleEstimates, 200);
    expect(estimate).toBe(3.2238805970149254);
    expect(Math.abs(estimate - Math.PI)).toBe(0.08228794342513224);
    expect(Math.abs(estimate - Math.PI)).toBeLessThan(0.25);
  });

  it("answers the rectangle's area exactly when the predicate always holds", () => {
    expect(
      streamTake(
        estimateIntegral(() => true, 0, 1, 0, 1),
        3,
      ),
    ).toEqual([1, 1, 1]);
  });

  it("answers zero when the predicate never holds", () => {
    expect(
      streamTake(
        estimateIntegral(() => false, 0, 1, 0, 1),
        3,
      ),
    ).toEqual([0, 0, 0]);
  });
});
