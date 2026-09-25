// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { nFoldSmooth, smooth } from "./ex_1_44.js";

const square = (x: number): number => x * x;

describe("exercise 1.44", () => {
  it("smooth(square, 0.01)(2) averages the three panels", () => {
    // (1.99^2 + 2^2 + 2.01^2) / 3
    expect(smooth(square, 0.01)(2)).toBe(4.000066666666666);
  });

  it("smoothing an affine function is the identity on the center point", () => {
    expect(smooth((x: number): number => x, 0.5)(3)).toBe(3);
  });

  it("nFoldSmooth(square, 2, 0.01)(2) smooths the already smoothed", () => {
    expect(nFoldSmooth(square, 2, 0.01)(2)).toBe(4.000133333333333);
  });

  it("zero-fold smoothing returns f itself", () => {
    expect(nFoldSmooth(square, 0, 0.01)(3)).toBe(9);
  });

  it("smoothing pulls a maximum down toward its neighborhood average", () => {
    // sin has its maximum at pi/2; two smoothings at dx = 0.01 stay just under 1
    const smoothed = nFoldSmooth(Math.sin, 2, 0.01)(Math.PI / 2);
    expect(smoothed).toBe(0.9999333349999796);
    expect(smoothed).toBeLessThan(1);
    expect(smoothed).toBeGreaterThan(0.9999);
  });
});
