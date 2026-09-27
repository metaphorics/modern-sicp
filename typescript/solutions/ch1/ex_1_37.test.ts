// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { contFrac, contFracIter, smallestKForFourDecimals } from "./ex_1_37.js";

const one = (): number => 1.0;
const INV_PHI = 1 / ((1 + Math.sqrt(5)) / 2);

describe("exercise 1.37", () => {
  it("the all-ones fraction closes in on 1/phi", () => {
    expect(contFrac(one, one, 8)).toBe(0.6176470588235294);
    expect(contFrac(one, one, 10)).toBe(0.6179775280898876);
    expect(contFrac(one, one, 12)).toBe(0.6180257510729613);
    expect(contFrac(one, one, 14)).toBe(0.6180327868852459);
  });

  it("the loop twin agrees term by term", () => {
    for (let k = 1; k <= 30; k += 1) {
      expect(contFracIter(one, one, k)).toBe(contFrac(one, one, k));
    }
  });

  it("ten terms buy four decimal places", () => {
    expect(smallestKForFourDecimals()).toBe(10);
    expect(Math.abs(contFracIter(one, one, 10) - INV_PHI)).toBeLessThan(1e-4);
    expect(Math.abs(contFracIter(one, one, 9) - INV_PHI)).toBeGreaterThanOrEqual(1e-4);
  });
});
