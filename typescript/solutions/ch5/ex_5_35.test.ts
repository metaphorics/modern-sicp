// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_35 } from "./ex_5_35.ts";

describe("exercise 5.35 reverse-engineering the compiled figure", () => {
  it("renders the compiled statements of the figure's expression", () => {
    const lines = ex_5_35();
    expect(lines[0]).toContain("function f(x: number)");
    expect(lines[1]).toBe("figure reproduced:");
    expect(lines.length).toBeGreaterThan(3);
    expect(lines.some((line) => line.includes("save"))).toBe(true);
  });
});
