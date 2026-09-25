// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import fc from "fast-check";
import { describe, expect, it } from "vitest";

import { cubeRoot } from "./ex_1_08.js";

describe("exercise 1.8", () => {
  it("cube roots land on the printed values", () => {
    expect(cubeRoot(27)).toBeCloseTo(3, 4);
    expect(cubeRoot(8)).toBeCloseTo(2, 6);
    expect(cubeRoot(0.001)).toBeCloseTo(0.1, 6);
  });

  it("the relative test carries large radicands too", () => {
    expect(Math.abs(cubeRoot(1e9) - 1000) < 0.001 * 1000).toBe(true);
  });

  it("the cube of the result reproduces the radicand for generated inputs", () => {
    fc.assert(
      fc.property(fc.integer({ min: 1, max: 1_000_000 }), (x) => {
        const root = cubeRoot(x);
        expect(Math.abs(root * root * root - x) < 0.01 * x).toBe(true);
      }),
    );
  });
});
