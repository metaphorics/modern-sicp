// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_03 } from "./ex_5_03.ts";

describe("exercise 5.3 square-root controllers", () => {
  it("primitive and expanded machines converge to the square root", () => {
    const result = ex_5_03(2);
    expect(result.primitive).toBeCloseTo(Math.sqrt(2), 2);
    expect(result.expanded).toBeCloseTo(Math.sqrt(2), 2);
  });
  it("both designs reach the same tolerance boundary", () => {
    const result = ex_5_03(16);
    expect(Math.abs(result.primitive * result.primitive - 16)).toBeLessThan(0.001);
    expect(Math.abs(result.expanded * result.expanded - 16)).toBeLessThan(0.001);
  });
});
