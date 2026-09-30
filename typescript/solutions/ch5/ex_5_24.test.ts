// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_24 } from "./ex_5_24.ts";

describe("exercise 5.24 switch as a basic controller form", () => {
  it("selects the matching clause and the default", () => {
    const lines = ex_5_24();
    expect(lines.some((line) => line.includes("zero"))).toBe(true);
    expect(lines.some((line) => line.includes("one"))).toBe(true);
    expect(lines.some((line) => line.includes("many"))).toBe(true);
  });
  it("reports selected-body effects without relying on implicit fallthrough", () => {
    const lines = ex_5_24();
    expect(lines).toContain("11");
    expect(lines).toContain("10");
    expect(lines).toContain("100");
  });
  it("evaluates a clause test that is itself a variable read", () => {
    const lines = ex_5_24();
    expect(lines.some((line) => line.includes("matched"))).toBe(true);
  });
});
