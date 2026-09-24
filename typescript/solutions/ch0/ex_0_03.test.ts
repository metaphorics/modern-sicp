// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { recursionDepth, sumToIterative, sumToRecursive } from "./ex_0_03.js";

describe("exercise 0.3", () => {
  it("both versions agree on inputs the recursion survives", () => {
    expect(sumToRecursive(100)).toBe(5050);
    expect(sumToIterative(100)).toBe(5050);
  });

  it("the loop runs where the recursion is refused", () => {
    expect(sumToIterative(100_000)).toBe(5_000_050_000);
    expect(() => sumToRecursive(100_000)).toThrow(RangeError);
  });

  it("the floor is finite and well above a thousand frames", () => {
    const depth = recursionDepth();
    expect(Number.isFinite(depth)).toBe(true);
    expect(depth).toBeGreaterThan(1000);
  });
});
