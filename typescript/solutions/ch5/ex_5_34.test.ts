// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_34 } from "./ex_5_34.js";

describe("exercise 5.34", () => {
  it("annotates the bounded stack of the iterative factorial", () => {
    const answers = ex_5_34();
    expect(answers[0]).toContain("iter");
    expect(answers[1]).toMatch(/\d+ statements, \d+ saves/);
    expect(answers[2]).toContain("bounded");
  });
});
