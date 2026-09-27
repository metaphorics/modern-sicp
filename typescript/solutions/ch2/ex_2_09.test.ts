// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  addInterval,
  addWidthLaw,
  divInterval,
  mulInterval,
  subInterval,
  width,
} from "./ex_2_09.js";

describe("exercise 2.9", () => {
  it("the width of a sum and of a difference is the sum of the widths", () => {
    const x = { lo: 2, hi: 6 };
    const y = { lo: 10, hi: 14 };
    expect(addWidthLaw(x, y)).toBe(true);
    expect(width(addInterval(x, y))).toBe(4);
    expect(width(subInterval(x, y))).toBe(4);
    expect(width(x) + width(y)).toBe(4);
  });

  it("equal-width factors give different product widths", () => {
    const a = { lo: 2, hi: 6 };
    const b1 = { lo: 10, hi: 14 };
    const b2 = { lo: 100, hi: 104 };
    expect(width(b1)).toBe(width(b2));
    expect(width(mulInterval(a, b1))).toBe(32);
    expect(width(mulInterval(a, b2))).toBe(212);
  });

  it("equal-width divisors give different quotient widths too", () => {
    const a = { lo: 2, hi: 6 };
    const b1 = { lo: 10, hi: 14 };
    const b2 = { lo: 100, hi: 104 };
    expect(width(divInterval(a, b1))).toBeCloseTo(0.2285714285714286, 12);
    expect(width(divInterval(a, b2))).toBeCloseTo(0.020384615384615383, 12);
  });
});
