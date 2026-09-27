// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { center, makeCenterPercent, percent } from "./ex_2_12.js";

describe("exercise 2.12", () => {
  it("a 5-percent interval around 100 round-trips exactly", () => {
    const i = makeCenterPercent(100, 5);
    expect(i.lo).toBe(95);
    expect(i.hi).toBe(105);
    expect(percent(i)).toBe(5);
  });

  it("the center selector is unchanged and the tolerance reads back", () => {
    const i = makeCenterPercent(3.5, 15 / 3.5);
    expect(center(i)).toBe(3.5);
    expect(percent(i)).toBeCloseTo(15 / 3.5, 10);
  });
});
