// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { addInterval, lowerBound, makeInterval, upperBound } from "./ex_2_07.js";

describe("exercise 2.7", () => {
  it("the selectors read back what makeInterval glued", () => {
    const i = makeInterval(3.35, 3.65);
    expect(lowerBound(i)).toBe(3.35);
    expect(upperBound(i)).toBe(3.65);
  });

  it("the sum of intervals lands its bounds where the selectors say", () => {
    const s = addInterval(makeInterval(6.12, 7.48), makeInterval(4.465, 4.935));
    expect(lowerBound(s)).toBeCloseTo(10.585, 10);
    expect(upperBound(s)).toBeCloseTo(12.415, 10);
  });
});
