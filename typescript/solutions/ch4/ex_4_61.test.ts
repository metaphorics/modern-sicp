// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_4_61, nextToAnswers } from "./ex_4_61.js";

describe("exercise 4.61: next-to", () => {
  it("finds nested and repeated adjacent values in the input order", () => {
    const [nested, repeated] = nextToAnswers();
    expect(nested).toHaveLength(2);
    expect(nested.some((answer) => answer.includes("1 next-to (2 3)"))).toBe(true);
    expect(nested.some((answer) => answer.includes("(2 3) next-to 4"))).toBe(true);
    expect(repeated).toHaveLength(2);
    expect(repeated.some((answer) => answer.includes("2 next-to 1"))).toBe(true);
    expect(repeated.some((answer) => answer.includes("3 next-to 1"))).toBe(true);
  });

  it("reports both result sequences", () => {
    expect(ex_4_61()).toContain("next-to");
  });
});
