// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_51 } from "./ex_5_51.ts";

describe("exercise 5.51", { timeout: 120_000 }, () => {
  it("builds the C evaluator and its transcript matches the direct evaluator", () => {
    const lines = ex_5_51();
    expect(lines.length).toBeGreaterThan(0);
    expect(lines.some((line) => line.includes("120"))).toBe(true);
    expect(lines.some((line) => line.startsWith("Error:"))).toBe(false);
  });
});
