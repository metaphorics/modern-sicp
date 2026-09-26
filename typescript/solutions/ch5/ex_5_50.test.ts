// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_50 } from "./ex_5_50.js";

describe("exercise 5.50", { timeout: 600_000 }, () => {
  it("compiles the metacircular evaluator and prices the levels", () => {
    const answers = ex_5_50();
    expect(answers[0]).toContain("120");
    expect(answers[0]).toContain("(tick tick tick)");
    const l0 = Number(answers[1]?.split("= ")[1]);
    const l2 = Number(answers[3]?.split("= ")[1]);
    expect(l2).toBeGreaterThan(100 * Math.max(l0, 1));
  });
});
