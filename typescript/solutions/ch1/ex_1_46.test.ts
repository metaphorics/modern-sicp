// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { fixedPointIterativeImprove, iterativeImprove, sqrtIterativeImprove } from "./ex_1_46.js";

describe("exercise 1.46", () => {
  it("sqrtIterativeImprove(2) lands within the 0.001 tolerance of the root", () => {
    expect(sqrtIterativeImprove(2)).toBe(1.4142156862745097);
    expect(Math.abs(sqrtIterativeImprove(2) - Math.sqrt(2))).toBeLessThan(0.001);
  });

  it("sqrtIterativeImprove reproduces the 1.1.7 pins", () => {
    expect(sqrtIterativeImprove(4)).toBe(2.0000000929222947);
    expect(sqrtIterativeImprove(9)).toBe(3.00009155413138);
  });

  it("fixedPointIterativeImprove(cos, 1) finds the cosine fixed point", () => {
    expect(fixedPointIterativeImprove(Math.cos, 1.0)).toBe(0.7390893414033928);
  });

  it("fixedPointIterativeImprove solves x^x = 1000 from the guess 2", () => {
    const y = fixedPointIterativeImprove((g) => Math.log(1000) / Math.log(g), 2.0);
    expect(y).toBe(4.555540912917957);
    expect(y ** y).toBeCloseTo(1000, 1);
  });

  it("each call starts fresh from its own guess", () => {
    const halver = iterativeImprove(
      (guess) => guess < 0.001,
      (guess) => guess / 2,
    );
    // 64 halves down to the first power of two below 0.001
    expect(halver(64)).toBe(0.0009765625);
    // a guess already below 0.001 comes back with no improve step
    expect(halver(0.0005)).toBe(0.0005);
  });
});
