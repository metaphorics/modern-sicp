// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { argumentLists, ex_5_36 } from "./ex_5_36.ts";

describe("exercise 5.36 operand evaluation order", () => {
  it("both orderings construct the same argument list at equal size", () => {
    const { rightToLeft, leftToRight } = argumentLists();
    expect(rightToLeft).toHaveLength(leftToRight.length);
    expect(rightToLeft[0]).toEqual(leftToRight[0]);
  });
  it("reports both sizes and the shipped listing", () => {
    const lines = ex_5_36();
    expect(lines[0]).toContain("right-to-left statements:");
    expect(lines[1]).toContain("left-to-right statements:");
    expect(lines[3]).toContain("shipped listing");
  });
});
