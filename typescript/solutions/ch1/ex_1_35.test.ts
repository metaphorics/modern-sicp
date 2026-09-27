// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { goldenRatio } from "./ex_1_35.js";

describe("exercise 1.35", () => {
  it("converges on the fixed point", () => {
    expect(goldenRatio()).toBe(1.6180327868852458);
  });

  it("satisfies the defining equation phi^2 = phi + 1", () => {
    const phi = goldenRatio();
    expect(Math.abs(phi * phi - phi - 1)).toBeLessThan(1e-4);
  });

  it("satisfies the fixed-point equation phi = 1 + 1/phi within tolerance", () => {
    const phi = goldenRatio();
    expect(Math.abs(phi - (1 + 1 / phi))).toBeLessThan(1e-4);
  });
});
