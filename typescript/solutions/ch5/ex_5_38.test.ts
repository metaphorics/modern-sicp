// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_38 } from "./ex_5_38.js";

describe("exercise 5.38", () => {
  it("pins the open-coded sizes and all the reproducer answers", () => {
    const answers = ex_5_38();
    expect(answers[0]).toContain("open-coded");
    expect(answers[1]).toContain("10");
    expect(answers[1]).toContain("#t");
    expect(answers[2]).toContain("15");
    expect(answers[2]).toContain("43");
    expect(answers[2]).toContain("10");
    expect(answers[3]).toContain("45");
    expect(answers[3]).toContain("44");
    expect(answers[4]).toContain("14");
    expect(answers[4]).toContain("17");
    expect(answers[5]).toContain("120");
    expect(answers[6]).toContain("22");
  });
});
