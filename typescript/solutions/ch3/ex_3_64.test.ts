// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { sqrtStream, streamRef } from "../../packages/ch3/src/05-streams.js";

import {
  acceleratedSqrtStream,
  sqrt2AcceleratedLimit,
  sqrtWithin,
  streamLimitWithTerms,
} from "./ex_3_64.js";

describe("exercise 3.64: stream-limit convergence helper", () => {
  it("computes sqrt 2 within 0.00001", () => {
    expect(sqrtWithin(2, 0.00001)).toBe(1.4142135623746899);
    expect(Math.abs(sqrtWithin(2, 0.00001) - Math.sqrt(2))).toBeLessThan(0.00001);
  });

  it("computes sqrt 3 within 0.00001", () => {
    expect(sqrtWithin(3, 0.00001)).toBe(1.7320508075688772);
    expect(sqrtWithin(3, 0.00001)).toBe(Math.sqrt(3));
  });

  it("computes sqrt 325 within 0.00001", () => {
    expect(sqrtWithin(325, 0.00001)).toBe(18.027756377319946);
    expect(sqrtWithin(325, 0.00001)).toBe(Math.sqrt(325));
  });

  it("counts the Newton guesses each root needs", () => {
    expect(streamLimitWithTerms(sqrtStream(2), 0.00001)).toEqual({
      value: 1.4142135623746899,
      terms: 5,
    });
    expect(streamLimitWithTerms(sqrtStream(3), 0.00001)).toEqual({
      value: 1.7320508075688772,
      terms: 6,
    });
    expect(streamLimitWithTerms(sqrtStream(325), 0.00001)).toEqual({
      value: 18.027756377319946,
      terms: 10,
    });
  });

  it("accelerating the Newton stream saves one term at 1e-5, then degenerates", () => {
    expect(streamLimitWithTerms(acceleratedSqrtStream(2), 0.00001)).toEqual({
      value: sqrt2AcceleratedLimit,
      terms: 4,
    });
    expect(streamLimitWithTerms(acceleratedSqrtStream(2), 0.00001).value).toBeCloseTo(
      Math.sqrt(2),
      15,
    );
    expect(Number.isNaN(streamRef(acceleratedSqrtStream(2), 4))).toBe(true);
  });
});
