// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_03 } from "./ex_5_03.js";

describe("exercise 5.3 square-root controllers", () => {
  it("primitive and expanded machines converge to the square root", () => {
    const result = ex_5_03(2);
    expect(result.primitive.ok && result.primitive.value.registers["guess"]).toBeCloseTo(
      Math.sqrt(2),
      2,
    );
    expect(result.expanded.ok && result.expanded.value.registers["guess"]).toBeCloseTo(
      Math.sqrt(2),
      2,
    );
  });
});
