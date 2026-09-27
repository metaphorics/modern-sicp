// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_36 } from "./ex_5_36.js";

describe("exercise 5.36", () => {
  it("evaluates operands right to left by default and left to right flipped", () => {
    const answers = ex_5_36();
    expect(answers[0]).toContain("[2,1]");
    expect(answers[1]).toContain("[1,2]");
    expect(answers[2]).toContain("/");
  });
});
