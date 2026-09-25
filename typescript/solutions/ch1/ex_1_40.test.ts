// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { cubic, newtonsMethod } from "./ex_1_40.js";

describe("exercise 1.40", () => {
  it("cubic evaluates x^3 + ax^2 + bx + c", () => {
    expect(cubic(1, 2, 3)(2)).toBe(19);
    expect(cubic(-6, 11, -6)(1)).toBe(0);
    expect(cubic(0, 0, 0)(5)).toBe(125);
  });

  it("Newton's method lands on the integer root of (x-1)(x-2)(x-3)", () => {
    const root = newtonsMethod(cubic(-6, 11, -6), 1.5);
    expect(root).toBeCloseTo(3, 12);
  });

  it("a non-integer zero is found to the search tolerance", () => {
    const g = cubic(0, -2, -3);
    const root = newtonsMethod(g, 1.0);
    expect(root).toBeCloseTo(1.8932891963046012, 12);
    expect(Math.abs(g(root))).toBeLessThan(1e-6);
  });
});
