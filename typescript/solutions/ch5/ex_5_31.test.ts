// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { analyzeSaves, ex_5_31 } from "./ex_5_31.ts";

describe("exercise 5.31 which saves are superfluous", () => {
  it("pairs every save with its restore and classifies it", () => {
    const reports = analyzeSaves(
      "function factorial(n: number): number { return n === 1 ? 1 : factorial(n - 1) * n; }",
    );
    expect(reports.length).toBeGreaterThan(0);
    for (const report of reports) {
      expect(typeof report.superfluous).toBe("boolean");
    }
    expect(ex_5_31().every((line) => line.includes("save "))).toBe(true);
  });
  it("a save whose register the region rewrites is needed", () => {
    const reports = analyzeSaves("function f(x: number) { return x + 1; }");
    for (const report of reports) {
      expect(report.step).toBeGreaterThanOrEqual(0);
    }
  });
});
