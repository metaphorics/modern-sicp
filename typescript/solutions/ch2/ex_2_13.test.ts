// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { makeCenterPercent, productPercent } from "./ex_2_13.js";

describe("exercise 2.13", () => {
  it("the product's tolerance sits within 0.01 of the sum of the factors'", () => {
    const p = makeCenterPercent(10, 1);
    const q = makeCenterPercent(20, 2);
    const computed = productPercent(p, q);
    expect(computed).toBeCloseTo(2.9994001199759976, 12);
    expect(Math.abs(computed - 3)).toBeLessThan(0.01);
  });

  it("the gap shrinks as the tolerances shrink", () => {
    const small = Math.abs(
      productPercent(makeCenterPercent(10, 0.1), makeCenterPercent(20, 0.2)) - 0.3,
    );
    const large = Math.abs(
      productPercent(makeCenterPercent(10, 5), makeCenterPercent(20, 10)) - 15,
    );
    expect(small).toBeLessThan(large);
  });
});
