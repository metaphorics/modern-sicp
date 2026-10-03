// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_49, readCompileExecutePrint } from "./ex_5_49.ts";

describe("exercise 5.49 read-compile-execute-print loop", () => {
  it("compiles and runs each form in order", () => {
    const lines = ex_5_49();
    expect(lines.length).toBeGreaterThan(0);
    expect(lines.some((line) => line.includes("36"))).toBe(true);
  });
  it("reports a failed form and keeps looping", () => {
    const lines = readCompileExecutePrint(["1 + 1;", "nonsense +++;", "2 + 2;"]);
    expect(lines.some((line) => line.startsWith("Error:"))).toBe(true);
    expect(lines.some((line) => line.includes("4"))).toBe(true);
  });
});
