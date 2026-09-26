// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_42 } from "./ex_5_42.js";

describe("exercise 5.42", () => {
  it("compiles and runs the lexical example", () => {
    const answers = ex_5_42();
    expect(answers[0]).toContain("x=(2 0)");
    expect(answers[1]).toContain("234");
  });
});
