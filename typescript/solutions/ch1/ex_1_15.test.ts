// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { sine, sineWithCount } from "./ex_1_15.js";

describe("exercise 1.15", () => {
  it("p is applied five times for 12.15", () => {
    expect(sineWithCount(12.15).pApplications).toBe(5);
  });

  it("the reduced value matches the standard library's sine", () => {
    expect(sine(12.15)).toBeCloseTo(Math.sin(12.15), 2);
    expect(sineWithCount(12.15).value).toBeCloseTo(-0.39980345741334, 12);
  });

  it("the count grows logarithmically with the angle", () => {
    expect(sineWithCount(0.05).pApplications).toBe(0);
    expect(sineWithCount(121.5).pApplications).toBe(7);
    expect(sineWithCount(1215).pApplications).toBe(9);
  });
});
