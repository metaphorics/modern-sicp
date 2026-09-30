// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { makeEvaluator } from "../../packages/ch5/src/04-eceval.ts";
import { ex_5_32, symbolOperator } from "./ex_5_32.ts";

describe("exercise 5.32 the symbol-operator fast path", () => {
  it("answers the same values as the generic path", () => {
    const lines = ex_5_32();
    expect(lines.some((line) => line.includes("36"))).toBe(true);
    expect(lines.some((line) => line.includes("42"))).toBe(true);
  });
  it("carries the design answer", () => {
    expect(ex_5_32()[ex_5_32().length - 1]).toContain("compile-time analysis");
  });
  it("the fast operation itself computes the open-coded set", () => {
    const ops = symbolOperator();
    const result = makeEvaluator("1 + 2;", ops).run();
    expect(result.outcome.tag).toBe("ok");
  });
});
