// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { simpson } from "./ex_1_29.js";

const cube = (x: number): number => x * x * x;

describe("exercise 1.29", () => {
  it("integrates cube over [0, 1] onto 1/4", () => {
    expect(simpson(cube, 0, 1, 100)).toBe(0.25000000000000006);
    expect(simpson(cube, 0, 1, 1000)).toBe(0.25000000000000006);
  });

  it("lands within float rounding of 1/4 at both panel counts", () => {
    expect(simpson(cube, 0, 1, 100)).toBeCloseTo(0.25, 12);
    expect(simpson(cube, 0, 1, 1000)).toBeCloseTo(0.25, 12);
  });

  it("beats the section's rectangular integral on the same subdivision", () => {
    const rectangular = 0.24998750000000042;
    expect(Math.abs(simpson(cube, 0, 1, 100) - 0.25)).toBeLessThan(Math.abs(rectangular - 0.25));
  });
});
