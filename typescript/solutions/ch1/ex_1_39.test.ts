// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { tanCf } from "./ex_1_39.js";

describe("exercise 1.39", () => {
  it("ten terms land on tan(1) to the last recorded digit", () => {
    expect(tanCf(1.0, 10)).toBe(1.557407724654902);
    expect(tanCf(1.0, 10)).toBeCloseTo(Math.tan(1.0), 12);
  });

  it("five terms are already at seven digits for tan(1)", () => {
    expect(tanCf(1.0, 5)).toBe(1.5574074074074076);
    expect(tanCf(1.0, 5)).toBeCloseTo(Math.tan(1.0), 6);
  });

  it("the fraction works across signs and magnitudes", () => {
    expect(tanCf(2.0, 50)).toBeCloseTo(Math.tan(2.0), 11);
    expect(tanCf(-1.0, 20)).toBeCloseTo(Math.tan(-1.0), 12);
    expect(tanCf(0.5, 10)).toBeCloseTo(Math.tan(0.5), 12);
  });
});
