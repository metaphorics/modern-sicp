// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_12 } from "./ex_5_12.ts";

describe("exercise 5.12 the assembler summary", () => {
  it("groups the gcd controller by type and lists its registers and labels", () => {
    const lines = ex_5_12();
    const groups = lines.filter((line) => line.includes(": "));
    expect(groups.some((line) => line.startsWith("test: "))).toBe(true);
    expect(groups.some((line) => line.startsWith("branch: "))).toBe(true);
    expect(groups.some((line) => line.startsWith("goto-label: "))).toBe(true);
    expect(lines.find((line) => line.startsWith("registers: "))).toContain("a b t");
    expect(lines.find((line) => line.startsWith("labels: "))).toContain("test-b gcd-done");
    const tSource = lines.find((line) => line.startsWith("t <- "));
    expect(tSource).toBeDefined();
    expect(tSource ?? "").toContain("rem");
  });
  it("the gcd controller saves nothing and jumps by label", () => {
    const lines = ex_5_12();
    expect(lines.find((line) => line.startsWith("saved:"))).toBe("saved:");
    expect(lines.find((line) => line.startsWith("entry-points:"))).toBe("entry-points:");
  });
});
